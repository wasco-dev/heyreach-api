use crate::bindings::exports::wasco_dev::heyreach_api::heyreach_api::{
    AuthError, MutationError, QueryError, ResourceError,
};
use crate::bindings::wasi::http::outgoing_handler;
use crate::bindings::wasi::http::types::*;
use crate::bindings::wasi::io::streams::StreamError;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub enum HttpMethod {
    Get,
    Post,
    Delete,
}

pub enum HttpError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Validation(String),
    TooManyRequests(String),
    Unknown(String),
}

impl HttpError {
    pub fn message(self) -> String {
        match self {
            HttpError::BadRequest(message) => message,
            HttpError::Unauthorized(message) => message,
            HttpError::NotFound(message) => message,
            HttpError::Validation(message) => message,
            HttpError::TooManyRequests(message) => message,
            HttpError::Unknown(message) => message,
        }
    }
}

impl From<HttpError> for AuthError {
    fn from(error: HttpError) -> Self {
        match error {
            HttpError::Unauthorized(message) => AuthError::Unauthorized(message),
            HttpError::TooManyRequests(message) => AuthError::TooManyRequests(message),
            other => AuthError::Unknown(other.message()),
        }
    }
}

impl From<HttpError> for QueryError {
    fn from(error: HttpError) -> Self {
        match error {
            HttpError::BadRequest(message) => QueryError::BadRequest(message),
            HttpError::Unauthorized(message) => QueryError::Unauthorized(message),
            HttpError::Validation(message) => QueryError::Validation(message),
            HttpError::TooManyRequests(message) => QueryError::TooManyRequests(message),
            other => QueryError::Unknown(other.message()),
        }
    }
}

impl From<HttpError> for ResourceError {
    fn from(error: HttpError) -> Self {
        match error {
            HttpError::Unauthorized(message) => ResourceError::Unauthorized(message),
            HttpError::NotFound(message) => ResourceError::NotFound(message),
            HttpError::TooManyRequests(message) => ResourceError::TooManyRequests(message),
            other => ResourceError::Unknown(other.message()),
        }
    }
}

impl From<HttpError> for MutationError {
    fn from(error: HttpError) -> Self {
        match error {
            HttpError::BadRequest(message) => MutationError::BadRequest(message),
            HttpError::Unauthorized(message) => MutationError::Unauthorized(message),
            HttpError::NotFound(message) => MutationError::NotFound(message),
            HttpError::Validation(message) => MutationError::Validation(message),
            HttpError::TooManyRequests(message) => MutationError::TooManyRequests(message),
            other => MutationError::Unknown(other.message()),
        }
    }
}

pub fn send_request_and_deserialize<T: DeserializeOwned>(
    method: HttpMethod,
    path: &str,
    api_key: &str,
    body: Option<&impl Serialize>,
) -> Result<T, HttpError> {
    let response = send_http_request(method, path, api_key, body)?;
    let status = response.status();
    let response_bytes = read_response_body(response)?;

    if status >= 400 {
        return Err(map_status_code_to_http_error(status, &response_bytes));
    }

    let response_text = String::from_utf8(response_bytes)
        .map_err(|error| HttpError::Unknown(format!("Invalid UTF-8 in response: {}", error)))?;

    serde_json::from_str(&response_text)
        .map_err(|error| HttpError::Unknown(format!("Failed to parse response: {}", error)))
}

pub fn send_request_without_response(
    method: HttpMethod,
    path: &str,
    api_key: &str,
    body: Option<&impl Serialize>,
) -> Result<(), HttpError> {
    let response = send_http_request(method, path, api_key, body)?;
    let status = response.status();

    if status >= 400 {
        let response_bytes = read_response_body(response)?;
        return Err(map_status_code_to_http_error(status, &response_bytes));
    }

    let _ = response.consume();
    Ok(())
}

fn send_http_request(
    method: HttpMethod,
    path: &str,
    api_key: &str,
    body: Option<&impl Serialize>,
) -> Result<IncomingResponse, HttpError> {
    let headers = Fields::new();

    headers
        .append("content-type", b"application/json; charset=utf-8")
        .map_err(|_| HttpError::Unknown("Failed to append content-type header".to_string()))?;

    headers
        .append("x-api-key", api_key.as_bytes())
        .map_err(|_| HttpError::Unknown("Failed to set API key header".to_string()))?;

    let outgoing_request = OutgoingRequest::new(headers);

    let method_value = match method {
        HttpMethod::Get => Method::Get,
        HttpMethod::Post => Method::Post,
        HttpMethod::Delete => Method::Delete,
    };

    outgoing_request
        .set_method(&method_value)
        .map_err(|_| HttpError::Unknown("Failed to set method".to_string()))?;
    outgoing_request
        .set_path_with_query(Some(path))
        .map_err(|_| HttpError::Unknown("Failed to set path".to_string()))?;
    outgoing_request
        .set_scheme(Some(&Scheme::Https))
        .map_err(|_| HttpError::Unknown("Failed to set scheme".to_string()))?;
    outgoing_request
        .set_authority(Some("api.heyreach.io"))
        .map_err(|_| HttpError::Unknown("Failed to set authority".to_string()))?;

    if let Some(body_data) = body {
        write_request_body(&outgoing_request, body_data)?;
    }

    let future_response = outgoing_handler::handle(outgoing_request, None)
        .map_err(|_| HttpError::Unknown("Failed to send request".to_string()))?;

    future_response.subscribe().block();

    future_response
        .get()
        .ok_or_else(|| HttpError::Unknown("Request not completed".to_string()))?
        .map_err(|_| HttpError::Unknown("Request failed".to_string()))?
        .map_err(|_| HttpError::Unknown("Request error".to_string()))
}

fn write_request_body(
    outgoing_request: &OutgoingRequest,
    body_data: &impl Serialize,
) -> Result<(), HttpError> {
    let body_bytes = serde_json::to_vec(body_data)
        .map_err(|error| HttpError::Unknown(format!("Failed to serialize body: {}", error)))?;

    let outgoing_body = outgoing_request
        .body()
        .map_err(|_| HttpError::Unknown("Failed to get outgoing body".to_string()))?;

    let body_stream = outgoing_body
        .write()
        .map_err(|_| HttpError::Unknown("Failed to get body stream".to_string()))?;

    body_stream
        .blocking_write_and_flush(&body_bytes)
        .map_err(|_| HttpError::Unknown("Failed to write body".to_string()))?;

    drop(body_stream);

    OutgoingBody::finish(outgoing_body, None)
        .map_err(|_| HttpError::Unknown("Failed to finish body".to_string()))
}

fn read_response_body(response: IncomingResponse) -> Result<Vec<u8>, HttpError> {
    let incoming_body = response
        .consume()
        .map_err(|_| HttpError::Unknown("Failed to get response body".to_string()))?;

    let body_stream = incoming_body
        .stream()
        .map_err(|_| HttpError::Unknown("Failed to get body stream".to_string()))?;

    let mut response_bytes = Vec::new();
    loop {
        match body_stream.blocking_read(8192) {
            Ok(chunk) if chunk.is_empty() => break,
            Ok(chunk) => response_bytes.extend_from_slice(&chunk),
            Err(StreamError::Closed) => break,
            Err(_) => return Err(HttpError::Unknown("Failed to read response".to_string())),
        }
    }

    drop(body_stream);
    Ok(response_bytes)
}

fn map_status_code_to_http_error(status: StatusCode, body_bytes: &[u8]) -> HttpError {
    let message = extract_error_message(status, body_bytes);

    match status {
        400 => HttpError::BadRequest(message),
        401 => HttpError::Unauthorized(message),
        404 => HttpError::NotFound(message),
        422 => HttpError::Validation(message),
        429 => HttpError::TooManyRequests(message),
        _ => HttpError::Unknown(message),
    }
}

fn extract_error_message(status: StatusCode, body_bytes: &[u8]) -> String {
    let error_message = String::from_utf8_lossy(body_bytes);

    serde_json::from_str::<serde_json::Value>(&error_message)
        .ok()
        .and_then(|json| {
            json.get("detail")
                .or_else(|| json.get("errorMessage"))
                .or_else(|| json.get("message"))
                .and_then(|value| value.as_str().map(String::from))
        })
        .unwrap_or_else(|| format!("HTTP {}", status))
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------- HttpError::message --------

    #[test]
    fn test_http_error_message_extracts_bad_request() {
        // Arrange
        let error = HttpError::BadRequest("bad input".to_string());

        // Act
        let message = error.message();

        // Assert
        assert_eq!(message, "bad input");
    }

    #[test]
    fn test_http_error_message_extracts_unauthorized() {
        // Arrange
        let error = HttpError::Unauthorized("not authorized".to_string());

        // Act
        let message = error.message();

        // Assert
        assert_eq!(message, "not authorized");
    }

    #[test]
    fn test_http_error_message_extracts_not_found() {
        // Arrange
        let error = HttpError::NotFound("missing".to_string());

        // Act
        let message = error.message();

        // Assert
        assert_eq!(message, "missing");
    }

    #[test]
    fn test_http_error_message_extracts_validation() {
        // Arrange
        let error = HttpError::Validation("invalid field".to_string());

        // Act
        let message = error.message();

        // Assert
        assert_eq!(message, "invalid field");
    }

    #[test]
    fn test_http_error_message_extracts_too_many_requests() {
        // Arrange
        let error = HttpError::TooManyRequests("slow down".to_string());

        // Act
        let message = error.message();

        // Assert
        assert_eq!(message, "slow down");
    }

    #[test]
    fn test_http_error_message_extracts_unknown() {
        // Arrange
        let error = HttpError::Unknown("something broke".to_string());

        // Act
        let message = error.message();

        // Assert
        assert_eq!(message, "something broke");
    }

    // -------- map_status_code_to_http_error: status code mapping --------

    #[test]
    fn test_map_status_code_to_http_error_400_bad_request() {
        // Arrange & Act
        let error = map_status_code_to_http_error(400, b"not json");

        // Assert
        assert!(matches!(error, HttpError::BadRequest(_)));
    }

    #[test]
    fn test_map_status_code_to_http_error_401_unauthorized() {
        // Arrange & Act
        let error = map_status_code_to_http_error(401, b"not json");

        // Assert
        assert!(matches!(error, HttpError::Unauthorized(_)));
    }

    #[test]
    fn test_map_status_code_to_http_error_404_not_found() {
        // Arrange & Act
        let error = map_status_code_to_http_error(404, b"not json");

        // Assert
        assert!(matches!(error, HttpError::NotFound(_)));
    }

    #[test]
    fn test_map_status_code_to_http_error_422_validation() {
        // Arrange & Act
        let error = map_status_code_to_http_error(422, b"not json");

        // Assert
        assert!(matches!(error, HttpError::Validation(_)));
    }

    #[test]
    fn test_map_status_code_to_http_error_429_too_many_requests() {
        // Arrange & Act
        let error = map_status_code_to_http_error(429, b"not json");

        // Assert
        assert!(matches!(error, HttpError::TooManyRequests(_)));
    }

    #[test]
    fn test_map_status_code_to_http_error_500_unknown() {
        // Arrange & Act
        let error = map_status_code_to_http_error(500, b"not json");

        // Assert
        assert!(matches!(error, HttpError::Unknown(_)));
    }

    // -------- extract_error_message: field priority --------

    #[test]
    fn test_extract_error_message_prefers_detail_field() {
        // Arrange
        let body = br#"{"detail": "API key is invalid"}"#;

        // Act
        let message = extract_error_message(401, body);

        // Assert
        assert_eq!(message, "API key is invalid");
    }

    #[test]
    fn test_extract_error_message_falls_back_to_error_message_field() {
        // Arrange
        let body = br#"{"errorMessage": "Rate limit exceeded"}"#;

        // Act
        let message = extract_error_message(429, body);

        // Assert
        assert_eq!(message, "Rate limit exceeded");
    }

    #[test]
    fn test_extract_error_message_falls_back_to_message_field() {
        // Arrange
        let body = br#"{"message": "Resource not found"}"#;

        // Act
        let message = extract_error_message(404, body);

        // Assert
        assert_eq!(message, "Resource not found");
    }

    #[test]
    fn test_extract_error_message_detail_takes_priority() {
        // Arrange
        let body = br#"{"detail": "from detail", "message": "from message"}"#;

        // Act
        let message = extract_error_message(400, body);

        // Assert
        assert_eq!(message, "from detail");
    }

    #[test]
    fn test_extract_error_message_falls_back_for_non_json() {
        // Arrange
        let body = b"plain text error";

        // Act
        let message = extract_error_message(500, body);

        // Assert
        assert_eq!(message, "HTTP 500");
    }

    #[test]
    fn test_extract_error_message_falls_back_for_json_without_recognized_keys() {
        // Arrange
        let body = br#"{"code": 500, "info": "something"}"#;

        // Act
        let message = extract_error_message(500, body);

        // Assert
        assert_eq!(message, "HTTP 500");
    }

    #[test]
    fn test_extract_error_message_falls_back_for_empty_body() {
        // Arrange & Act
        let message = extract_error_message(400, b"");

        // Assert
        assert_eq!(message, "HTTP 400");
    }

    // -------- From<HttpError> for AuthError --------

    #[test]
    fn test_http_error_unauthorized_converts_to_auth_error() {
        // Arrange
        let error = HttpError::Unauthorized("bad key".to_string());

        // Act
        let auth_error: AuthError = error.into();

        // Assert
        assert!(matches!(auth_error, AuthError::Unauthorized(message) if message == "bad key"));
    }

    #[test]
    fn test_http_error_too_many_requests_converts_to_auth_error() {
        // Arrange
        let error = HttpError::TooManyRequests("slow down".to_string());

        // Act
        let auth_error: AuthError = error.into();

        // Assert
        assert!(
            matches!(auth_error, AuthError::TooManyRequests(message) if message == "slow down")
        );
    }

    #[test]
    fn test_http_error_unknown_fallthrough_converts_to_auth_unknown() {
        // Arrange
        let error = HttpError::NotFound("not here".to_string());

        // Act
        let auth_error: AuthError = error.into();

        // Assert
        assert!(matches!(auth_error, AuthError::Unknown(message) if message == "not here"));
    }

    // -------- From<HttpError> for QueryError --------

    #[test]
    fn test_http_error_bad_request_converts_to_query_error() {
        // Arrange
        let error = HttpError::BadRequest("bad input".to_string());

        // Act
        let query_error: QueryError = error.into();

        // Assert
        assert!(matches!(query_error, QueryError::BadRequest(message) if message == "bad input"));
    }

    #[test]
    fn test_http_error_unauthorized_converts_to_query_error() {
        // Arrange
        let error = HttpError::Unauthorized("no auth".to_string());

        // Act
        let query_error: QueryError = error.into();

        // Assert
        assert!(matches!(query_error, QueryError::Unauthorized(message) if message == "no auth"));
    }

    #[test]
    fn test_http_error_validation_converts_to_query_error() {
        // Arrange
        let error = HttpError::Validation("bad field".to_string());

        // Act
        let query_error: QueryError = error.into();

        // Assert
        assert!(matches!(query_error, QueryError::Validation(message) if message == "bad field"));
    }

    #[test]
    fn test_http_error_too_many_requests_converts_to_query_error() {
        // Arrange
        let error = HttpError::TooManyRequests("rate limited".to_string());

        // Act
        let query_error: QueryError = error.into();

        // Assert
        assert!(
            matches!(query_error, QueryError::TooManyRequests(message) if message == "rate limited")
        );
    }

    #[test]
    fn test_http_error_unknown_fallthrough_converts_to_query_unknown() {
        // Arrange
        let error = HttpError::NotFound("not here".to_string());

        // Act
        let query_error: QueryError = error.into();

        // Assert
        assert!(matches!(query_error, QueryError::Unknown(message) if message == "not here"));
    }

    // -------- From<HttpError> for ResourceError --------

    #[test]
    fn test_http_error_unauthorized_converts_to_resource_error() {
        // Arrange
        let error = HttpError::Unauthorized("no auth".to_string());

        // Act
        let resource_error: ResourceError = error.into();

        // Assert
        assert!(
            matches!(resource_error, ResourceError::Unauthorized(message) if message == "no auth")
        );
    }

    #[test]
    fn test_http_error_not_found_converts_to_resource_error() {
        // Arrange
        let error = HttpError::NotFound("missing".to_string());

        // Act
        let resource_error: ResourceError = error.into();

        // Assert
        assert!(matches!(resource_error, ResourceError::NotFound(message) if message == "missing"));
    }

    #[test]
    fn test_http_error_too_many_requests_converts_to_resource_error() {
        // Arrange
        let error = HttpError::TooManyRequests("rate limited".to_string());

        // Act
        let resource_error: ResourceError = error.into();

        // Assert
        assert!(
            matches!(resource_error, ResourceError::TooManyRequests(message) if message == "rate limited")
        );
    }

    #[test]
    fn test_http_error_unknown_fallthrough_converts_to_resource_unknown() {
        // Arrange
        let error = HttpError::BadRequest("bad input".to_string());

        // Act
        let resource_error: ResourceError = error.into();

        // Assert
        assert!(
            matches!(resource_error, ResourceError::Unknown(message) if message == "bad input")
        );
    }

    // -------- From<HttpError> for MutationError --------

    #[test]
    fn test_http_error_bad_request_converts_to_mutation_error() {
        // Arrange
        let error = HttpError::BadRequest("bad input".to_string());

        // Act
        let mutation_error: MutationError = error.into();

        // Assert
        assert!(
            matches!(mutation_error, MutationError::BadRequest(message) if message == "bad input")
        );
    }

    #[test]
    fn test_http_error_unauthorized_converts_to_mutation_error() {
        // Arrange
        let error = HttpError::Unauthorized("no auth".to_string());

        // Act
        let mutation_error: MutationError = error.into();

        // Assert
        assert!(
            matches!(mutation_error, MutationError::Unauthorized(message) if message == "no auth")
        );
    }

    #[test]
    fn test_http_error_not_found_converts_to_mutation_error() {
        // Arrange
        let error = HttpError::NotFound("missing".to_string());

        // Act
        let mutation_error: MutationError = error.into();

        // Assert
        assert!(matches!(mutation_error, MutationError::NotFound(message) if message == "missing"));
    }

    #[test]
    fn test_http_error_validation_converts_to_mutation_error() {
        // Arrange
        let error = HttpError::Validation("bad field".to_string());

        // Act
        let mutation_error: MutationError = error.into();

        // Assert
        assert!(
            matches!(mutation_error, MutationError::Validation(message) if message == "bad field")
        );
    }

    #[test]
    fn test_http_error_too_many_requests_converts_to_mutation_error() {
        // Arrange
        let error = HttpError::TooManyRequests("rate limited".to_string());

        // Act
        let mutation_error: MutationError = error.into();

        // Assert
        assert!(
            matches!(mutation_error, MutationError::TooManyRequests(message) if message == "rate limited")
        );
    }

    #[test]
    fn test_http_error_unknown_fallthrough_converts_to_mutation_unknown() {
        // Arrange
        let error = HttpError::Unknown("something else".to_string());

        // Act
        let mutation_error: MutationError = error.into();

        // Assert
        assert!(
            matches!(mutation_error, MutationError::Unknown(message) if message == "something else")
        );
    }
}
