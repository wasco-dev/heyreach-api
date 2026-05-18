use crate::bindings::exports::wasco_dev::heyreach_api::heyreach_api::*;
use crate::http::{send_request_and_deserialize, send_request_without_response, HttpMethod};
use crate::models::*;

// -------- Helper functions for conversion --------

fn campaign_status_from_string(status: &str) -> CampaignStatus {
    match status.to_lowercase().as_str() {
        "draft" => CampaignStatus::Draft,
        "active" => CampaignStatus::Active,
        "paused" => CampaignStatus::Paused,
        "finished" => CampaignStatus::Finished,
        "canceled" => CampaignStatus::Canceled,
        _ => CampaignStatus::Unknown,
    }
}

fn campaign_status_to_string(status: &CampaignStatus) -> String {
    match status {
        CampaignStatus::Draft => "draft",
        CampaignStatus::Active => "active",
        CampaignStatus::Paused => "paused",
        CampaignStatus::Finished => "finished",
        CampaignStatus::Canceled => "canceled",
        CampaignStatus::Unknown => "unknown",
    }
    .to_string()
}

fn list_type_from_string(list_type: &str) -> ListType {
    match list_type.to_lowercase().as_str() {
        "leads" => ListType::Leads,
        "companies" => ListType::Companies,
        _ => ListType::Unknown,
    }
}

fn webhook_event_type_from_string(event_type: &str) -> WebhookEventType {
    match event_type.to_lowercase().as_str() {
        "connectionrequestsent" | "connection_request_sent" | "connection-request-sent" => {
            WebhookEventType::ConnectionRequestSent
        }
        "connectionaccepted" | "connection_accepted" | "connection-accepted" => {
            WebhookEventType::ConnectionAccepted
        }
        "messagesent" | "message_sent" | "message-sent" => WebhookEventType::MessageSent,
        "messagereplied" | "message_replied" | "message-replied" => {
            WebhookEventType::MessageReplied
        }
        _ => WebhookEventType::Unknown,
    }
}

fn webhook_event_type_to_string(event_type: &WebhookEventType) -> String {
    match event_type {
        WebhookEventType::ConnectionRequestSent => "ConnectionRequestSent",
        WebhookEventType::ConnectionAccepted => "ConnectionAccepted",
        WebhookEventType::MessageSent => "MessageSent",
        WebhookEventType::MessageReplied => "MessageReplied",
        WebhookEventType::Unknown => "Unknown",
    }
    .to_string()
}

fn progress_stats_from_dto(stats: ProgressStatsDto) -> ProgressStats {
    ProgressStats {
        total_users: stats.total_users,
        total_users_in_progress: stats.total_users_in_progress,
        total_users_pending: stats.total_users_pending,
        total_users_finished: stats.total_users_finished,
        total_users_failed: stats.total_users_failed,
        total_users_manually_stopped: stats.total_users_manually_stopped,
        total_users_excluded: stats.total_users_excluded,
    }
}

fn campaign_summary_from_dto(campaign: CampaignSummaryDto) -> CampaignSummary {
    CampaignSummary {
        id: campaign.id,
        name: campaign.name,
        creation_time: campaign.creation_time,
        linkedin_user_list_name: campaign.linkedin_user_list_name,
        linkedin_user_list_id: campaign.linkedin_user_list_id,
        campaign_account_ids: campaign.campaign_account_ids,
        status: campaign_status_from_string(&campaign.status),
        progress_stats: campaign.progress_stats.map(progress_stats_from_dto),
        exclude_already_messaged_global: campaign.exclude_already_messaged_global,
        exclude_already_messaged_campaign_accounts: campaign
            .exclude_already_messaged_campaign_accounts,
        exclude_first_connection_campaign_accounts: campaign
            .exclude_first_connection_campaign_accounts,
        exclude_first_connection_global: campaign.exclude_first_connection_global,
        exclude_no_profile_picture: campaign.exclude_no_profile_picture,
        exclude_list_id: campaign.exclude_list_id,
        exclude_in_other_campaigns: campaign.exclude_in_other_campaigns,
        exclude_has_other_acc_conversations: campaign.exclude_has_other_acc_conversations,
        exclude_contacted_from_sender_in_other_campaign: campaign
            .exclude_contacted_from_sender_in_other_campaign,
        organization_unit_id: campaign.organization_unit_id,
    }
}

fn lead_from_dto(lead: LeadDto) -> Lead {
    Lead {
        first_name: lead.first_name,
        last_name: lead.last_name,
        profile_url: lead.profile_url,
        location: lead.location,
        summary: lead.summary,
        company_name: lead.company_name,
        position: lead.position,
        about: lead.about,
        email_address: lead.email_address,
        custom_user_fields: lead
            .custom_user_fields
            .into_iter()
            .map(|field| CustomUserField {
                name: field.name,
                value: field.value,
            })
            .collect(),
    }
}

fn lead_to_dto(lead: Lead) -> LeadDto {
    LeadDto {
        first_name: lead.first_name,
        last_name: lead.last_name,
        profile_url: lead.profile_url,
        location: lead.location,
        summary: lead.summary,
        company_name: lead.company_name,
        position: lead.position,
        about: lead.about,
        email_address: lead.email_address,
        custom_user_fields: lead
            .custom_user_fields
            .into_iter()
            .map(|field| CustomUserFieldDto {
                name: field.name,
                value: field.value,
            })
            .collect(),
    }
}

fn list_summary_from_dto(list: ListSummaryDto) -> ListSummary {
    ListSummary {
        id: list.id,
        name: list.name,
        total_items_count: list.total_items_count,
        list_type: list_type_from_string(&list.list_type),
        creation_time: list.creation_time,
        campaign_ids: list.campaign_ids,
    }
}

fn webhook_from_dto(webhook: WebhookDto) -> Webhook {
    Webhook {
        id: webhook.id,
        webhook_name: webhook.webhook_name,
        webhook_url: webhook.webhook_url,
        event_type: webhook_event_type_from_string(&webhook.event_type),
        campaign_ids: webhook.campaign_ids,
        is_active: webhook.is_active,
    }
}

// -------- Auth --------

pub fn check_api_key(api_key: &str) -> Result<(), ApiError> {
    send_request_without_response(
        HttpMethod::Get,
        "/api/public/auth/CheckApiKey",
        api_key,
        None::<&()>,
    )
}

// -------- Campaigns --------

pub fn campaigns_get_all(api_key: &str, filter: CampaignFilter) -> Result<CampaignPage, ApiError> {
    let filter_dto = CampaignFilterDto {
        offset: filter.offset,
        limit: filter.limit,
        keyword: filter.keyword,
        statuses: filter
            .statuses
            .iter()
            .map(campaign_status_to_string)
            .collect(),
        account_ids: filter.account_ids,
    };

    let response: CampaignPageDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/campaign/GetAll",
        api_key,
        Some(&filter_dto),
    )?;

    Ok(CampaignPage {
        total_count: response.total_count,
        items: response
            .items
            .into_iter()
            .map(campaign_summary_from_dto)
            .collect(),
    })
}

pub fn campaigns_get_by_id(api_key: &str, campaign_id: u64) -> Result<CampaignSummary, ApiError> {
    let response: CampaignSummaryDto = send_request_and_deserialize(
        HttpMethod::Get,
        &format!("/api/public/campaign/GetById?campaignId={}", campaign_id),
        api_key,
        None::<&()>,
    )?;

    Ok(campaign_summary_from_dto(response))
}

pub fn campaigns_resume(api_key: &str, campaign_id: u64) -> Result<(), ApiError> {
    send_request_without_response(
        HttpMethod::Post,
        &format!("/api/public/campaign/Resume?campaignId={}", campaign_id),
        api_key,
        None::<&()>,
    )
}

pub fn campaigns_pause(api_key: &str, campaign_id: u64) -> Result<(), ApiError> {
    send_request_without_response(
        HttpMethod::Post,
        &format!("/api/public/campaign/Pause?campaignId={}", campaign_id),
        api_key,
        None::<&()>,
    )
}

pub fn campaigns_add_leads(
    api_key: &str,
    payload: CampaignAddLeadsRequest,
) -> Result<u32, ApiError> {
    let payload_dto = CampaignAddLeadsRequestDto {
        campaign_id: payload.campaign_id,
        account_lead_pairs: payload
            .account_lead_pairs
            .into_iter()
            .map(|pair| AccountLeadPairDto {
                linked_in_account_id: pair.linked_in_account_id,
                lead: lead_to_dto(pair.lead),
            })
            .collect(),
    };

    send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/campaign/AddLeadsToCampaign",
        api_key,
        Some(&payload_dto),
    )
}

pub fn campaigns_add_leads_v2(
    api_key: &str,
    payload: CampaignAddLeadsRequest,
) -> Result<CampaignAddLeadsV2Result, ApiError> {
    let payload_dto = CampaignAddLeadsRequestDto {
        campaign_id: payload.campaign_id,
        account_lead_pairs: payload
            .account_lead_pairs
            .into_iter()
            .map(|pair| AccountLeadPairDto {
                linked_in_account_id: pair.linked_in_account_id,
                lead: lead_to_dto(pair.lead),
            })
            .collect(),
    };

    let response: CampaignAddLeadsV2ResultDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/campaign/AddLeadsToCampaignV2",
        api_key,
        Some(&payload_dto),
    )?;

    Ok(CampaignAddLeadsV2Result {
        added_leads_count: response.added_leads_count,
        updated_leads_count: response.updated_leads_count,
        failed_leads_count: response.failed_leads_count,
    })
}

// -------- Lists --------

pub fn lists_get_all(api_key: &str, filter: ListGetAllFilter) -> Result<ListPage, ApiError> {
    let filter_dto = ListGetAllFilterDto {
        offset: filter.offset,
        limit: filter.limit,
        keyword: filter.keyword,
    };

    let response: ListPageDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/list/GetAll",
        api_key,
        Some(&filter_dto),
    )?;

    Ok(ListPage {
        total_count: response.total_count,
        items: response
            .items
            .into_iter()
            .map(list_summary_from_dto)
            .collect(),
    })
}

pub fn lists_get_by_id(api_key: &str, list_id: u64) -> Result<ListSummary, ApiError> {
    let response: ListSummaryDto = send_request_and_deserialize(
        HttpMethod::Get,
        &format!("/api/public/list/GetById?listId={}", list_id),
        api_key,
        None::<&()>,
    )?;

    Ok(list_summary_from_dto(response))
}

pub fn lists_get_leads(
    api_key: &str,
    list_id: u64,
    offset: u32,
    limit: u32,
    keyword: Option<String>,
) -> Result<ListLeadsPage, ApiError> {
    let request_dto = ListGetLeadsRequestDto {
        list_id,
        offset,
        limit,
        keyword,
    };

    let response: ListLeadsPageDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/list/GetLeadsFromList",
        api_key,
        Some(&request_dto),
    )?;

    Ok(ListLeadsPage {
        total_count: response.total_count,
        items: response.items.into_iter().map(lead_from_dto).collect(),
    })
}

pub fn lists_add_leads(api_key: &str, list_id: u64, leads: Vec<Lead>) -> Result<(), ApiError> {
    let request_dto = ListAddLeadsRequestDto {
        list_id,
        leads: leads.into_iter().map(lead_to_dto).collect(),
    };

    send_request_without_response(
        HttpMethod::Post,
        "/api/public/list/AddLeadsToList",
        api_key,
        Some(&request_dto),
    )
}

pub fn lists_add_leads_v2(
    api_key: &str,
    list_id: u64,
    leads: Vec<Lead>,
) -> Result<CampaignAddLeadsV2Result, ApiError> {
    let request_dto = ListAddLeadsRequestDto {
        list_id,
        leads: leads.into_iter().map(lead_to_dto).collect(),
    };

    let response: CampaignAddLeadsV2ResultDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/list/AddLeadsToListV2",
        api_key,
        Some(&request_dto),
    )?;

    Ok(CampaignAddLeadsV2Result {
        added_leads_count: response.added_leads_count,
        updated_leads_count: response.updated_leads_count,
        failed_leads_count: response.failed_leads_count,
    })
}

pub fn lists_delete_leads(api_key: &str, request: ListLeadDeleteRequest) -> Result<(), ApiError> {
    let request_dto = ListLeadDeleteRequestDto {
        list_id: request.list_id,
        lead_member_ids: request.lead_member_ids,
    };

    send_request_without_response(
        HttpMethod::Delete,
        "/api/public/list/DeleteLeadsFromList",
        api_key,
        Some(&request_dto),
    )
}

pub fn lists_delete_leads_by_profile_url(
    api_key: &str,
    request: ListLeadDeleteByProfileUrlRequest,
) -> Result<ListLeadDeleteByProfileUrlResponse, ApiError> {
    let request_dto = ListLeadDeleteByProfileUrlRequestDto {
        list_id: request.list_id,
        profile_urls: request.profile_urls,
    };

    let response: ListLeadDeleteByProfileUrlResponseDto = send_request_and_deserialize(
        HttpMethod::Delete,
        "/api/public/list/DeleteLeadsFromListByProfileUrl",
        api_key,
        Some(&request_dto),
    )?;

    Ok(ListLeadDeleteByProfileUrlResponse {
        not_found_in_list: response.not_found_in_list,
    })
}

// -------- Lead & Tags --------

pub fn lead_get(api_key: &str, profile_url: String) -> Result<Lead, ApiError> {
    let request_dto = LeadGetRequestDto { profile_url };

    let response: LeadDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/lead/GetLead",
        api_key,
        Some(&request_dto),
    )?;

    Ok(lead_from_dto(response))
}

pub fn lead_get_lists(
    api_key: &str,
    request: LeadListsRequest,
) -> Result<LeadListsResponse, ApiError> {
    let request_dto = LeadListsRequestDto {
        email: request.email,
        linkedin_id: request.linkedin_id,
        profile_url: request.profile_url,
        offset: request.offset,
        limit: request.limit,
    };

    let response: LeadListsResponseDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/list/GetListsForLead",
        api_key,
        Some(&request_dto),
    )?;

    Ok(LeadListsResponse {
        total_count: response.total_count,
        items: response
            .items
            .into_iter()
            .map(|lead_list| LeadListSummary {
                list_id: lead_list.list_id,
                list_name: lead_list.list_name,
            })
            .collect(),
    })
}

pub fn lead_get_tags(api_key: &str, profile_url: String) -> Result<LeadTagsResponse, ApiError> {
    let request_dto = LeadGetRequestDto { profile_url };

    let response: LeadTagsResponseDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/lead/GetTags",
        api_key,
        Some(&request_dto),
    )?;

    Ok(LeadTagsResponse {
        tags: response.tags,
    })
}

pub fn lead_replace_tags(
    api_key: &str,
    request: LeadReplaceTagsRequest,
) -> Result<LeadReplaceTagsResponse, ApiError> {
    let request_dto = LeadReplaceTagsRequestDto {
        lead_profile_url: request.lead_profile_url,
        lead_linked_in_id: request.lead_linked_in_id,
        tags: request.tags,
        create_tag_if_not_existing: request.create_tag_if_not_existing,
    };

    let response: LeadReplaceTagsResponseDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/lead/ReplaceTags",
        api_key,
        Some(&request_dto),
    )?;

    Ok(LeadReplaceTagsResponse {
        new_assigned_tags: response.new_assigned_tags,
    })
}

// -------- Inbox --------

pub fn inbox_get_conversations_v2(
    api_key: &str,
    request: InboxGetConversationsRequest,
) -> Result<InboxConversationPage, ApiError> {
    let request_dto = InboxGetConversationsRequestDto {
        filters: InboxFiltersDto {
            linked_in_account_ids: request.filters.linked_in_account_ids,
            campaign_ids: request.filters.campaign_ids,
            search_string: request.filters.search_string,
            lead_linked_in_id: request.filters.lead_linked_in_id,
            lead_profile_url: request.filters.lead_profile_url,
            seen: request.filters.seen,
        },
        offset: request.offset,
        limit: request.limit,
    };

    let response: InboxConversationPageDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/inbox/GetConversationsV2",
        api_key,
        Some(&request_dto),
    )?;

    Ok(InboxConversationPage {
        total_count: response.total_count,
        items: response
            .items
            .into_iter()
            .map(|conversation| InboxConversationSummary {
                conversation_id: conversation.conversation_id,
                linked_in_account_id: conversation.linked_in_account_id,
                lead_profile_url: conversation.lead_profile_url,
                last_message_snippet: conversation.last_message_snippet,
                seen: conversation.seen,
            })
            .collect(),
    })
}

pub fn inbox_send_message(api_key: &str, request: InboxSendMessageRequest) -> Result<(), ApiError> {
    let request_dto = InboxSendMessageRequestDto {
        message: request.message,
        subject: request.subject,
        conversation_id: request.conversation_id,
        linked_in_account_id: request.linked_in_account_id,
    };

    send_request_without_response(
        HttpMethod::Post,
        "/api/public/inbox/SendMessage",
        api_key,
        Some(&request_dto),
    )
}

// -------- LinkedIn Accounts --------

pub fn li_account_get_all(
    api_key: &str,
    filter: LiAccountFilter,
) -> Result<LiAccountPage, ApiError> {
    let filter_dto = LiAccountFilterDto {
        offset: filter.offset,
        limit: filter.limit,
        keyword: filter.keyword,
    };

    let response: LiAccountPageDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/li_account/GetAll",
        api_key,
        Some(&filter_dto),
    )?;

    Ok(LiAccountPage {
        total_count: response.total_count,
        items: response
            .items
            .into_iter()
            .map(|account| LiAccountSummary {
                id: account.id,
                email_address: account.email_address,
                first_name: account.first_name,
                last_name: account.last_name,
                is_active: account.is_active,
                active_campaigns: account.active_campaigns,
                auth_is_valid: account.auth_is_valid,
                is_valid_navigator: account.is_valid_navigator,
                is_valid_recruiter: account.is_valid_recruiter,
            })
            .collect(),
    })
}

// -------- Webhooks --------

pub fn webhooks_create(api_key: &str, request: CreateWebhookRequest) -> Result<Webhook, ApiError> {
    let request_dto = CreateWebhookRequestDto {
        webhook_name: request.webhook_name,
        webhook_url: request.webhook_url,
        event_type: webhook_event_type_to_string(&request.event_type),
        campaign_ids: request.campaign_ids,
        is_active: request.is_active,
    };

    let response: WebhookDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/webhooks/CreateWebhook",
        api_key,
        Some(&request_dto),
    )?;

    Ok(webhook_from_dto(response))
}

pub fn webhooks_get_by_id(api_key: &str, webhook_id: u64) -> Result<Webhook, ApiError> {
    let response: WebhookDto = send_request_and_deserialize(
        HttpMethod::Get,
        &format!(
            "/api/public/webhooks/GetWebhookById?webhookId={}",
            webhook_id
        ),
        api_key,
        None::<&()>,
    )?;

    Ok(webhook_from_dto(response))
}

pub fn webhooks_get_all(api_key: &str, filter: GetWebhooksFilter) -> Result<WebhookPage, ApiError> {
    let filter_dto = GetWebhooksFilterDto {
        offset: filter.offset,
        limit: filter.limit,
    };

    let response: WebhookPageDto = send_request_and_deserialize(
        HttpMethod::Post,
        "/api/public/webhooks/GetAllWebhooks",
        api_key,
        Some(&filter_dto),
    )?;

    Ok(WebhookPage {
        total_count: response.total_count,
        items: response.items.into_iter().map(webhook_from_dto).collect(),
    })
}

pub fn webhooks_delete(api_key: &str, webhook_id: u64) -> Result<(), ApiError> {
    send_request_without_response(
        HttpMethod::Delete,
        &format!(
            "/api/public/webhooks/DeleteWebhook?webhookId={}",
            webhook_id
        ),
        api_key,
        None::<&()>,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------- campaign_status_from_string --------

    #[test]
    fn test_campaign_status_from_string_valid_statuses() {
        // Arrange & Act & Assert
        assert!(matches!(
            campaign_status_from_string("draft"),
            CampaignStatus::Draft
        ));
        assert!(matches!(
            campaign_status_from_string("active"),
            CampaignStatus::Active
        ));
        assert!(matches!(
            campaign_status_from_string("paused"),
            CampaignStatus::Paused
        ));
        assert!(matches!(
            campaign_status_from_string("finished"),
            CampaignStatus::Finished
        ));
        assert!(matches!(
            campaign_status_from_string("canceled"),
            CampaignStatus::Canceled
        ));
    }

    #[test]
    fn test_campaign_status_from_string_unknown() {
        // Arrange & Act & Assert
        assert!(matches!(
            campaign_status_from_string("invalid"),
            CampaignStatus::Unknown
        ));
        assert!(matches!(
            campaign_status_from_string(""),
            CampaignStatus::Unknown
        ));
    }

    #[test]
    fn test_campaign_status_from_string_case_insensitive() {
        // Arrange & Act & Assert
        assert!(matches!(
            campaign_status_from_string("DRAFT"),
            CampaignStatus::Draft
        ));
        assert!(matches!(
            campaign_status_from_string("Active"),
            CampaignStatus::Active
        ));
        assert!(matches!(
            campaign_status_from_string("PAUSED"),
            CampaignStatus::Paused
        ));
        assert!(matches!(
            campaign_status_from_string("Finished"),
            CampaignStatus::Finished
        ));
        assert!(matches!(
            campaign_status_from_string("CANCELED"),
            CampaignStatus::Canceled
        ));
    }

    // -------- campaign_status_to_string --------

    #[test]
    fn test_campaign_status_to_string_all_variants() {
        // Arrange & Act & Assert
        assert_eq!(campaign_status_to_string(&CampaignStatus::Draft), "draft");
        assert_eq!(campaign_status_to_string(&CampaignStatus::Active), "active");
        assert_eq!(campaign_status_to_string(&CampaignStatus::Paused), "paused");
        assert_eq!(
            campaign_status_to_string(&CampaignStatus::Finished),
            "finished"
        );
        assert_eq!(
            campaign_status_to_string(&CampaignStatus::Canceled),
            "canceled"
        );
        assert_eq!(
            campaign_status_to_string(&CampaignStatus::Unknown),
            "unknown"
        );
    }

    #[test]
    fn test_campaign_status_round_trip() {
        // Arrange
        let statuses = vec![
            CampaignStatus::Draft,
            CampaignStatus::Active,
            CampaignStatus::Paused,
            CampaignStatus::Finished,
            CampaignStatus::Canceled,
        ];

        for status in &statuses {
            // Act
            let as_string = campaign_status_to_string(status);
            let back = campaign_status_from_string(&as_string);

            // Assert
            assert!(matches!(
                (&status, &back),
                (CampaignStatus::Draft, CampaignStatus::Draft)
                    | (CampaignStatus::Active, CampaignStatus::Active)
                    | (CampaignStatus::Paused, CampaignStatus::Paused)
                    | (CampaignStatus::Finished, CampaignStatus::Finished)
                    | (CampaignStatus::Canceled, CampaignStatus::Canceled)
            ));
        }
    }

    // -------- list_type_from_string --------

    #[test]
    fn test_list_type_from_string_valid_types() {
        // Arrange & Act & Assert
        assert!(matches!(list_type_from_string("leads"), ListType::Leads));
        assert!(matches!(
            list_type_from_string("companies"),
            ListType::Companies
        ));
    }

    #[test]
    fn test_list_type_from_string_unknown() {
        // Arrange & Act & Assert
        assert!(matches!(
            list_type_from_string("invalid"),
            ListType::Unknown
        ));
        assert!(matches!(list_type_from_string(""), ListType::Unknown));
    }

    #[test]
    fn test_list_type_from_string_case_insensitive() {
        // Arrange & Act & Assert
        assert!(matches!(list_type_from_string("LEADS"), ListType::Leads));
        assert!(matches!(
            list_type_from_string("Companies"),
            ListType::Companies
        ));
    }

    // -------- webhook_event_type_from_string --------

    #[test]
    fn test_webhook_event_type_from_string_pascal_case() {
        // Arrange & Act & Assert
        assert!(matches!(
            webhook_event_type_from_string("ConnectionRequestSent"),
            WebhookEventType::ConnectionRequestSent
        ));
        assert!(matches!(
            webhook_event_type_from_string("ConnectionAccepted"),
            WebhookEventType::ConnectionAccepted
        ));
        assert!(matches!(
            webhook_event_type_from_string("MessageSent"),
            WebhookEventType::MessageSent
        ));
        assert!(matches!(
            webhook_event_type_from_string("MessageReplied"),
            WebhookEventType::MessageReplied
        ));
    }

    #[test]
    fn test_webhook_event_type_from_string_snake_case() {
        // Arrange & Act & Assert
        assert!(matches!(
            webhook_event_type_from_string("connection_request_sent"),
            WebhookEventType::ConnectionRequestSent
        ));
        assert!(matches!(
            webhook_event_type_from_string("connection_accepted"),
            WebhookEventType::ConnectionAccepted
        ));
        assert!(matches!(
            webhook_event_type_from_string("message_sent"),
            WebhookEventType::MessageSent
        ));
        assert!(matches!(
            webhook_event_type_from_string("message_replied"),
            WebhookEventType::MessageReplied
        ));
    }

    #[test]
    fn test_webhook_event_type_from_string_kebab_case() {
        // Arrange & Act & Assert
        assert!(matches!(
            webhook_event_type_from_string("connection-request-sent"),
            WebhookEventType::ConnectionRequestSent
        ));
        assert!(matches!(
            webhook_event_type_from_string("connection-accepted"),
            WebhookEventType::ConnectionAccepted
        ));
        assert!(matches!(
            webhook_event_type_from_string("message-sent"),
            WebhookEventType::MessageSent
        ));
        assert!(matches!(
            webhook_event_type_from_string("message-replied"),
            WebhookEventType::MessageReplied
        ));
    }

    #[test]
    fn test_webhook_event_type_from_string_unknown() {
        // Arrange & Act & Assert
        assert!(matches!(
            webhook_event_type_from_string("invalid"),
            WebhookEventType::Unknown
        ));
        assert!(matches!(
            webhook_event_type_from_string(""),
            WebhookEventType::Unknown
        ));
    }

    // -------- webhook_event_type_to_string --------

    #[test]
    fn test_webhook_event_type_to_string_all_variants() {
        // Arrange & Act & Assert
        assert_eq!(
            webhook_event_type_to_string(&WebhookEventType::ConnectionRequestSent),
            "ConnectionRequestSent"
        );
        assert_eq!(
            webhook_event_type_to_string(&WebhookEventType::ConnectionAccepted),
            "ConnectionAccepted"
        );
        assert_eq!(
            webhook_event_type_to_string(&WebhookEventType::MessageSent),
            "MessageSent"
        );
        assert_eq!(
            webhook_event_type_to_string(&WebhookEventType::MessageReplied),
            "MessageReplied"
        );
        assert_eq!(
            webhook_event_type_to_string(&WebhookEventType::Unknown),
            "Unknown"
        );
    }

    // -------- lead_from_dto / lead_to_dto --------

    #[test]
    fn test_lead_round_trip_all_fields() {
        // Arrange
        let dto = LeadDto {
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            profile_url: "https://linkedin.com/in/janedoe".to_string(),
            location: Some("London".to_string()),
            summary: Some("Engineer".to_string()),
            company_name: Some("Acme".to_string()),
            position: Some("CTO".to_string()),
            about: Some("Builds things".to_string()),
            email_address: Some("jane@example.com".to_string()),
            custom_user_fields: vec![
                CustomUserFieldDto {
                    name: "source".to_string(),
                    value: "linkedin".to_string(),
                },
                CustomUserFieldDto {
                    name: "priority".to_string(),
                    value: "high".to_string(),
                },
            ],
        };

        // Act
        let lead = lead_from_dto(dto);
        let round_tripped = lead_to_dto(lead);

        // Assert
        assert_eq!(round_tripped.first_name, "Jane");
        assert_eq!(round_tripped.last_name, "Doe");
        assert_eq!(round_tripped.profile_url, "https://linkedin.com/in/janedoe");
        assert_eq!(round_tripped.location, Some("London".to_string()));
        assert_eq!(round_tripped.summary, Some("Engineer".to_string()));
        assert_eq!(round_tripped.company_name, Some("Acme".to_string()));
        assert_eq!(round_tripped.position, Some("CTO".to_string()));
        assert_eq!(round_tripped.about, Some("Builds things".to_string()));
        assert_eq!(
            round_tripped.email_address,
            Some("jane@example.com".to_string())
        );
        assert_eq!(round_tripped.custom_user_fields.len(), 2);
        assert_eq!(round_tripped.custom_user_fields[0].name, "source");
        assert_eq!(round_tripped.custom_user_fields[0].value, "linkedin");
        assert_eq!(round_tripped.custom_user_fields[1].name, "priority");
        assert_eq!(round_tripped.custom_user_fields[1].value, "high");
    }

    #[test]
    fn test_lead_from_dto_optional_fields_none() {
        // Arrange
        let dto = LeadDto {
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
            profile_url: "https://linkedin.com/in/johnsmith".to_string(),
            location: None,
            summary: None,
            company_name: None,
            position: None,
            about: None,
            email_address: None,
            custom_user_fields: vec![],
        };

        // Act
        let lead = lead_from_dto(dto);

        // Assert
        assert_eq!(lead.first_name, "John");
        assert_eq!(lead.last_name, "Smith");
        assert_eq!(lead.profile_url, "https://linkedin.com/in/johnsmith");
        assert!(lead.location.is_none());
        assert!(lead.summary.is_none());
        assert!(lead.company_name.is_none());
        assert!(lead.position.is_none());
        assert!(lead.about.is_none());
        assert!(lead.email_address.is_none());
        assert!(lead.custom_user_fields.is_empty());
    }

    // -------- progress_stats_from_dto --------

    #[test]
    fn test_progress_stats_from_dto() {
        // Arrange
        let dto = ProgressStatsDto {
            total_users: 100,
            total_users_in_progress: -5,
            total_users_pending: 20,
            total_users_finished: 50,
            total_users_failed: 10,
            total_users_manually_stopped: 8,
            total_users_excluded: 7,
        };

        // Act
        let stats = progress_stats_from_dto(dto);

        // Assert
        assert_eq!(stats.total_users, 100);
        assert_eq!(stats.total_users_in_progress, -5);
        assert_eq!(stats.total_users_pending, 20);
        assert_eq!(stats.total_users_finished, 50);
        assert_eq!(stats.total_users_failed, 10);
        assert_eq!(stats.total_users_manually_stopped, 8);
        assert_eq!(stats.total_users_excluded, 7);
    }

    // -------- list_summary_from_dto --------

    #[test]
    fn test_list_summary_from_dto() {
        // Arrange
        let dto = ListSummaryDto {
            id: 42,
            name: "My List".to_string(),
            total_items_count: 150,
            list_type: "leads".to_string(),
            creation_time: "2024-01-15T10:00:00Z".to_string(),
            campaign_ids: vec![1, 2, 3],
        };

        // Act
        let summary = list_summary_from_dto(dto);

        // Assert
        assert_eq!(summary.id, 42);
        assert_eq!(summary.name, "My List");
        assert_eq!(summary.total_items_count, 150);
        assert!(matches!(summary.list_type, ListType::Leads));
        assert_eq!(summary.creation_time, "2024-01-15T10:00:00Z");
        assert_eq!(summary.campaign_ids, vec![1, 2, 3]);
    }

    // -------- webhook_from_dto --------

    #[test]
    fn test_webhook_from_dto() {
        // Arrange
        let dto = WebhookDto {
            id: 99,
            webhook_name: "My Webhook".to_string(),
            webhook_url: "https://example.com/hook".to_string(),
            event_type: "ConnectionRequestSent".to_string(),
            campaign_ids: vec![10, 20],
            is_active: true,
        };

        // Act
        let webhook = webhook_from_dto(dto);

        // Assert
        assert_eq!(webhook.id, 99);
        assert_eq!(webhook.webhook_name, "My Webhook");
        assert_eq!(webhook.webhook_url, "https://example.com/hook");
        assert!(matches!(
            webhook.event_type,
            WebhookEventType::ConnectionRequestSent
        ));
        assert_eq!(webhook.campaign_ids, vec![10, 20]);
        assert!(webhook.is_active);
    }

    #[test]
    fn test_webhook_from_dto_unknown_event_type() {
        // Arrange
        let dto = WebhookDto {
            id: 1,
            webhook_name: "Hook".to_string(),
            webhook_url: "https://example.com".to_string(),
            event_type: "SomeNewEvent".to_string(),
            campaign_ids: vec![],
            is_active: false,
        };

        // Act
        let webhook = webhook_from_dto(dto);

        // Assert
        assert!(matches!(webhook.event_type, WebhookEventType::Unknown));
        assert!(!webhook.is_active);
    }

    // -------- campaign_summary_from_dto --------

    #[test]
    fn test_campaign_summary_from_dto_with_progress_stats() {
        // Arrange
        let stats_dto = ProgressStatsDto {
            total_users: 50,
            total_users_in_progress: 10,
            total_users_pending: 15,
            total_users_finished: 20,
            total_users_failed: 3,
            total_users_manually_stopped: 1,
            total_users_excluded: 1,
        };
        let dto = CampaignSummaryDto {
            id: 7,
            name: "Test Campaign".to_string(),
            creation_time: "2024-06-01T12:00:00Z".to_string(),
            linkedin_user_list_name: Some("Prospects".to_string()),
            linkedin_user_list_id: Some(123),
            campaign_account_ids: vec![1, 2],
            status: "active".to_string(),
            progress_stats: Some(stats_dto),
            exclude_in_other_campaigns: true,
            exclude_has_other_acc_conversations: false,
            exclude_contacted_from_sender_in_other_campaign: true,
            exclude_list_id: Some(456),
            organization_unit_id: Some(789),
            exclude_already_messaged_global: Some(true),
            exclude_already_messaged_campaign_accounts: None,
            exclude_first_connection_campaign_accounts: Some(false),
            exclude_first_connection_global: None,
            exclude_no_profile_picture: Some(true),
        };

        // Act
        let summary = campaign_summary_from_dto(dto);

        // Assert
        assert_eq!(summary.id, 7);
        assert_eq!(summary.name, "Test Campaign");
        assert_eq!(summary.creation_time, "2024-06-01T12:00:00Z");
        assert!(matches!(summary.status, CampaignStatus::Active));
        assert_eq!(
            summary.linkedin_user_list_name,
            Some("Prospects".to_string())
        );
        assert_eq!(summary.linkedin_user_list_id, Some(123));
        assert_eq!(summary.campaign_account_ids, vec![1, 2]);
        assert!(summary.progress_stats.is_some());
        let stats = summary.progress_stats.unwrap();
        assert_eq!(stats.total_users, 50);
        assert_eq!(stats.total_users_in_progress, 10);
        assert!(summary.exclude_in_other_campaigns);
        assert!(!summary.exclude_has_other_acc_conversations);
        assert!(summary.exclude_contacted_from_sender_in_other_campaign);
        assert_eq!(summary.exclude_list_id, Some(456));
        assert_eq!(summary.organization_unit_id, Some(789));
        assert_eq!(summary.exclude_already_messaged_global, Some(true));
        assert!(summary.exclude_already_messaged_campaign_accounts.is_none());
        assert_eq!(
            summary.exclude_first_connection_campaign_accounts,
            Some(false)
        );
        assert!(summary.exclude_first_connection_global.is_none());
        assert_eq!(summary.exclude_no_profile_picture, Some(true));
    }

    #[test]
    fn test_campaign_summary_from_dto_without_progress_stats() {
        // Arrange
        let dto = CampaignSummaryDto {
            id: 1,
            name: "Empty Campaign".to_string(),
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            linkedin_user_list_name: None,
            linkedin_user_list_id: None,
            campaign_account_ids: vec![],
            status: "draft".to_string(),
            progress_stats: None,
            exclude_in_other_campaigns: false,
            exclude_has_other_acc_conversations: false,
            exclude_contacted_from_sender_in_other_campaign: false,
            exclude_list_id: None,
            organization_unit_id: None,
            exclude_already_messaged_global: None,
            exclude_already_messaged_campaign_accounts: None,
            exclude_first_connection_campaign_accounts: None,
            exclude_first_connection_global: None,
            exclude_no_profile_picture: None,
        };

        // Act
        let summary = campaign_summary_from_dto(dto);

        // Assert
        assert!(matches!(summary.status, CampaignStatus::Draft));
        assert!(summary.progress_stats.is_none());
        assert!(summary.linkedin_user_list_name.is_none());
        assert!(summary.linkedin_user_list_id.is_none());
        assert!(summary.campaign_account_ids.is_empty());
    }
}
