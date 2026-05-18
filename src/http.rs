use crate::bindings::exports::wasco_dev::heyreach_api::heyreach_api::{ApiError, ApiErrorCode};
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

pub fn send_request_and_deserialize<T: DeserializeOwned>(
    method: HttpMethod,
    path: &str,
    api_key: &str,
    body: Option<&impl Serialize>,
) -> Result<T, ApiError> {
    let response = send_http_request(method, path, api_key, body)?;
    let status = response.status();
    let response_bytes = read_response_body(response)?;

    if status >= 400 {
        return Err(map_http_status_to_error(status, &response_bytes));
    }

    let response_text = String::from_utf8(response_bytes).map_err(|error| {
        build_api_error(ApiErrorCode::Unknown, &format!("Invalid UTF-8 in response: {}", error))
    })?;

    serde_json::from_str(&response_text).map_err(|error| {
        build_api_error(
            ApiErrorCode::Unknown,
            &format!("Failed to parse response: {}", error),
        )
    })
}

pub fn send_request_without_response(
    method: HttpMethod,
    path: &str,
    api_key: &str,
    body: Option<&impl Serialize>,
) -> Result<(), ApiError> {
    let response = send_http_request(method, path, api_key, body)?;
    let status = response.status();

    if status >= 400 {
        let response_bytes = read_response_body(response)?;
        return Err(map_http_status_to_error(status, &response_bytes));
    }

    let _ = response.consume();
    Ok(())
}

fn send_http_request(
    method: HttpMethod,
    path: &str,
    api_key: &str,
    body: Option<&impl Serialize>,
) -> Result<IncomingResponse, ApiError> {
    let headers = Fields::new();

    headers
        .append(
            &"content-type".to_string(),
            &b"application/json; charset=utf-8".to_vec(),
        )
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to append content-type header"))?;

    headers
        .append(&"x-api-key".to_string(), api_key.as_bytes())
        .map_err(|_| build_api_error(ApiErrorCode::Unauthorized, "Failed to set API key header"))?;

    let outgoing_request = OutgoingRequest::new(headers);

    let method_value = match method {
        HttpMethod::Get => Method::Get,
        HttpMethod::Post => Method::Post,
        HttpMethod::Delete => Method::Delete,
    };

    outgoing_request
        .set_method(&method_value)
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to set method"))?;
    outgoing_request
        .set_path_with_query(Some(path))
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to set path"))?;
    outgoing_request
        .set_scheme(Some(&Scheme::Https))
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to set scheme"))?;
    outgoing_request
        .set_authority(Some("api.heyreach.io"))
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to set authority"))?;

    if let Some(body_data) = body {
        write_request_body(&outgoing_request, body_data)?;
    }

    let future_response = outgoing_handler::handle(outgoing_request, None)
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to send request"))?;

    future_response.subscribe().block();

    future_response
        .get()
        .ok_or_else(|| build_api_error(ApiErrorCode::Unknown, "Request not completed"))?
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Request failed"))?
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Request error"))
}

fn write_request_body(
    outgoing_request: &OutgoingRequest,
    body_data: &impl Serialize,
) -> Result<(), ApiError> {
    let body_bytes = serde_json::to_vec(body_data).map_err(|error| {
        build_api_error(
            ApiErrorCode::BadRequest,
            &format!("Failed to serialize body: {}", error),
        )
    })?;

    let outgoing_body = outgoing_request
        .body()
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to get outgoing body"))?;

    let body_stream = outgoing_body
        .write()
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to get body stream"))?;

    body_stream
        .blocking_write_and_flush(&body_bytes)
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to write body"))?;

    drop(body_stream);

    OutgoingBody::finish(outgoing_body, None)
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to finish body"))
}

fn read_response_body(response: IncomingResponse) -> Result<Vec<u8>, ApiError> {
    let incoming_body = response
        .consume()
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to get response body"))?;

    let body_stream = incoming_body
        .stream()
        .map_err(|_| build_api_error(ApiErrorCode::Unknown, "Failed to get body stream"))?;

    let mut response_bytes = Vec::new();
    loop {
        match body_stream.blocking_read(8192) {
            Ok(chunk) if chunk.is_empty() => break,
            Ok(chunk) => response_bytes.extend_from_slice(&chunk),
            Err(StreamError::Closed) => break,
            Err(_) => return Err(build_api_error(ApiErrorCode::Unknown, "Failed to read response")),
        }
    }

    drop(body_stream);
    Ok(response_bytes)
}

fn map_http_status_to_error(status: StatusCode, body_bytes: &[u8]) -> ApiError {
    let error_code = match status {
        400 => ApiErrorCode::BadRequest,
        401 => ApiErrorCode::Unauthorized,
        404 => ApiErrorCode::NotFound,
        422 => ApiErrorCode::Validation,
        429 => ApiErrorCode::TooManyRequests,
        _ => ApiErrorCode::Unknown,
    };

    let error_message = String::from_utf8_lossy(body_bytes);
    let message = serde_json::from_str::<serde_json::Value>(&error_message)
        .ok()
        .and_then(|json| {
            json.get("detail")
                .or_else(|| json.get("errorMessage"))
                .or_else(|| json.get("message"))
                .and_then(|value| value.as_str().map(String::from))
        })
        .unwrap_or_else(|| format!("HTTP {}", status));

    build_api_error(error_code, &message)
}

fn build_api_error(code: ApiErrorCode, message: &str) -> ApiError {
    ApiError {
        code,
        message: message.to_string(),
    }
}
