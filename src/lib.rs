mod client;
mod http;
mod models;

mod bindings {
    wit_bindgen::generate!({ generate_all });
    use crate::HeyreachApi;
    export!(HeyreachApi);
}

use bindings::exports::wasco_dev::heyreach_api::heyreach_api::*;

struct HeyreachApi;

impl Guest for HeyreachApi {
    // -------- Auth --------
    fn check_api_key(api_key: String) -> Result<(), ApiError> {
        client::check_api_key(&api_key)
    }

    // -------- Campaigns --------
    fn campaigns_get_all(
        api_key: String,
        filter: CampaignFilter,
    ) -> Result<CampaignPage, ApiError> {
        client::campaigns_get_all(&api_key, filter)
    }

    fn campaigns_get_by_id(api_key: String, campaign_id: u64) -> Result<CampaignSummary, ApiError> {
        client::campaigns_get_by_id(&api_key, campaign_id)
    }

    fn campaigns_resume(api_key: String, campaign_id: u64) -> Result<(), ApiError> {
        client::campaigns_resume(&api_key, campaign_id)
    }

    fn campaigns_pause(api_key: String, campaign_id: u64) -> Result<(), ApiError> {
        client::campaigns_pause(&api_key, campaign_id)
    }

    fn campaigns_add_leads(
        api_key: String,
        payload: CampaignAddLeadsRequest,
    ) -> Result<u32, ApiError> {
        client::campaigns_add_leads(&api_key, payload)
    }

    fn campaigns_add_leads_v2(
        api_key: String,
        payload: CampaignAddLeadsRequest,
    ) -> Result<CampaignAddLeadsV2Result, ApiError> {
        client::campaigns_add_leads_v2(&api_key, payload)
    }

    // -------- Lists --------
    fn lists_get_all(api_key: String, filter: ListGetAllFilter) -> Result<ListPage, ApiError> {
        client::lists_get_all(&api_key, filter)
    }

    fn lists_get_by_id(api_key: String, list_id: u64) -> Result<ListSummary, ApiError> {
        client::lists_get_by_id(&api_key, list_id)
    }

    fn lists_get_leads(
        api_key: String,
        list_id: u64,
        offset: u32,
        limit: u32,
        keyword: Option<String>,
    ) -> Result<ListLeadsPage, ApiError> {
        client::lists_get_leads(&api_key, list_id, offset, limit, keyword)
    }

    fn lists_add_leads(api_key: String, list_id: u64, leads: Vec<Lead>) -> Result<(), ApiError> {
        client::lists_add_leads(&api_key, list_id, leads)
    }

    fn lists_add_leads_v2(
        api_key: String,
        list_id: u64,
        leads: Vec<Lead>,
    ) -> Result<CampaignAddLeadsV2Result, ApiError> {
        client::lists_add_leads_v2(&api_key, list_id, leads)
    }

    fn lists_delete_leads(api_key: String, request: ListLeadDeleteRequest) -> Result<(), ApiError> {
        client::lists_delete_leads(&api_key, request)
    }

    fn lists_delete_leads_by_profile_url(
        api_key: String,
        request: ListLeadDeleteByProfileUrlRequest,
    ) -> Result<ListLeadDeleteByProfileUrlResponse, ApiError> {
        client::lists_delete_leads_by_profile_url(&api_key, request)
    }

    // -------- Lead & Tags --------
    fn lead_get(api_key: String, profile_url: String) -> Result<Lead, ApiError> {
        client::lead_get(&api_key, profile_url)
    }

    fn lead_get_lists(
        api_key: String,
        request: LeadListsRequest,
    ) -> Result<LeadListsResponse, ApiError> {
        client::lead_get_lists(&api_key, request)
    }

    fn lead_get_tags(api_key: String, profile_url: String) -> Result<LeadTagsResponse, ApiError> {
        client::lead_get_tags(&api_key, profile_url)
    }

    fn lead_replace_tags(
        api_key: String,
        request: LeadReplaceTagsRequest,
    ) -> Result<LeadReplaceTagsResponse, ApiError> {
        client::lead_replace_tags(&api_key, request)
    }

    // -------- Inbox --------
    fn inbox_get_conversations_v2(
        api_key: String,
        request: InboxGetConversationsRequest,
    ) -> Result<InboxConversationPage, ApiError> {
        client::inbox_get_conversations_v2(&api_key, request)
    }

    fn inbox_send_message(
        api_key: String,
        request: InboxSendMessageRequest,
    ) -> Result<(), ApiError> {
        client::inbox_send_message(&api_key, request)
    }

    // -------- LinkedIn Accounts --------
    fn li_account_get_all(
        api_key: String,
        filter: LiAccountFilter,
    ) -> Result<LiAccountPage, ApiError> {
        client::li_account_get_all(&api_key, filter)
    }

    // -------- Webhooks --------
    fn webhooks_create(
        api_key: String,
        request: CreateWebhookRequest,
    ) -> Result<Webhook, ApiError> {
        client::webhooks_create(&api_key, request)
    }

    fn webhooks_get_by_id(api_key: String, webhook_id: u64) -> Result<Webhook, ApiError> {
        client::webhooks_get_by_id(&api_key, webhook_id)
    }

    fn webhooks_get_all(
        api_key: String,
        filter: GetWebhooksFilter,
    ) -> Result<WebhookPage, ApiError> {
        client::webhooks_get_all(&api_key, filter)
    }

    fn webhooks_delete(api_key: String, webhook_id: u64) -> Result<(), ApiError> {
        client::webhooks_delete(&api_key, webhook_id)
    }
}
