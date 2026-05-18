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
        exclude_already_messaged_campaign_accounts: campaign.exclude_already_messaged_campaign_accounts,
        exclude_first_connection_campaign_accounts: campaign.exclude_first_connection_campaign_accounts,
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
        items: response
            .items
            .into_iter()
            .map(webhook_from_dto)
            .collect(),
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
