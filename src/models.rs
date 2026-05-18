use serde::{Deserialize, Serialize};

// -------- Campaigns --------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFilterDto {
    pub offset: u32,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    pub statuses: Vec<String>,
    pub account_ids: Vec<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressStatsDto {
    pub total_users: u32,
    pub total_users_in_progress: i32,
    pub total_users_pending: u32,
    pub total_users_finished: u32,
    pub total_users_failed: u32,
    #[serde(default)]
    pub total_users_manually_stopped: u32,
    #[serde(default)]
    pub total_users_excluded: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignSummaryDto {
    pub id: u64,
    pub name: String,
    pub creation_time: String,
    #[serde(rename = "linkedInUserListName")]
    pub linkedin_user_list_name: Option<String>,
    #[serde(rename = "linkedInUserListId")]
    pub linkedin_user_list_id: Option<u64>,
    pub campaign_account_ids: Vec<u32>,
    pub status: String,
    pub progress_stats: Option<ProgressStatsDto>,

    #[serde(default)]
    pub exclude_in_other_campaigns: bool,
    #[serde(default)]
    pub exclude_has_other_acc_conversations: bool,
    #[serde(default)]
    pub exclude_contacted_from_sender_in_other_campaign: bool,
    pub exclude_list_id: Option<u64>,
    #[serde(default)]
    pub organization_unit_id: Option<u64>,

    #[serde(skip_serializing)]
    pub exclude_already_messaged_global: Option<bool>,
    #[serde(skip_serializing)]
    pub exclude_already_messaged_campaign_accounts: Option<bool>,
    #[serde(skip_serializing)]
    pub exclude_first_connection_campaign_accounts: Option<bool>,
    #[serde(skip_serializing)]
    pub exclude_first_connection_global: Option<bool>,
    #[serde(skip_serializing)]
    pub exclude_no_profile_picture: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignPageDto {
    pub total_count: u32,
    pub items: Vec<CampaignSummaryDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomUserFieldDto {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadDto {
    pub first_name: String,
    pub last_name: String,
    pub profile_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default)]
    pub custom_user_fields: Vec<CustomUserFieldDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountLeadPairDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_in_account_id: Option<u32>,
    pub lead: LeadDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignAddLeadsRequestDto {
    pub campaign_id: u64,
    pub account_lead_pairs: Vec<AccountLeadPairDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignAddLeadsV2ResultDto {
    pub added_leads_count: u32,
    pub updated_leads_count: u32,
    pub failed_leads_count: u32,
}

// -------- Lists --------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGetAllFilterDto {
    pub offset: u32,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSummaryDto {
    pub id: u64,
    pub name: String,
    pub total_items_count: u32,
    pub list_type: String,
    pub creation_time: String,
    pub campaign_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPageDto {
    pub total_count: u32,
    pub items: Vec<ListSummaryDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLeadsPageDto {
    pub total_count: u32,
    pub items: Vec<LeadDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGetLeadsRequestDto {
    pub list_id: u64,
    pub offset: u32,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAddLeadsRequestDto {
    pub list_id: u64,
    pub leads: Vec<LeadDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLeadDeleteRequestDto {
    pub list_id: u64,
    pub lead_member_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLeadDeleteByProfileUrlRequestDto {
    pub list_id: u64,
    pub profile_urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLeadDeleteByProfileUrlResponseDto {
    pub not_found_in_list: Vec<String>,
}

// -------- Lead & Tags --------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadGetRequestDto {
    pub profile_url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadListsRequestDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_url: Option<String>,
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadListSummaryDto {
    pub list_id: u64,
    pub list_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadListsResponseDto {
    pub total_count: u32,
    pub items: Vec<LeadListSummaryDto>,
}

#[derive(Debug, Deserialize)]
pub struct LeadTagsResponseDto {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadReplaceTagsRequestDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_profile_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_linked_in_id: Option<String>,
    pub tags: Vec<String>,
    pub create_tag_if_not_existing: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadReplaceTagsResponseDto {
    pub new_assigned_tags: Vec<String>,
}

// -------- Inbox --------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxFiltersDto {
    pub linked_in_account_ids: Vec<u32>,
    pub campaign_ids: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_linked_in_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_profile_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxGetConversationsRequestDto {
    pub filters: InboxFiltersDto,
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxConversationSummaryDto {
    #[serde(rename = "id")]
    pub conversation_id: String,
    pub linked_in_account_id: u32,
    pub lead_profile_url: Option<String>,
    pub last_message_snippet: Option<String>,
    #[serde(rename = "read")]
    pub seen: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxConversationPageDto {
    pub total_count: u32,
    pub items: Vec<InboxConversationSummaryDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxSendMessageRequestDto {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "id")]
    pub conversation_id: String,
    pub linked_in_account_id: u32,
}

// -------- LinkedIn Accounts --------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiAccountFilterDto {
    pub offset: u32,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiAccountSummaryDto {
    pub id: u32,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub active_campaigns: u32,
    #[serde(default)]
    pub auth_is_valid: bool,
    #[serde(default)]
    pub is_valid_navigator: bool,
    #[serde(default)]
    pub is_valid_recruiter: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiAccountPageDto {
    pub total_count: u32,
    pub items: Vec<LiAccountSummaryDto>,
}

// -------- Webhooks --------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDto {
    pub id: u64,
    pub webhook_name: String,
    pub webhook_url: String,
    pub event_type: String,
    pub campaign_ids: Vec<u64>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookRequestDto {
    pub webhook_name: String,
    pub webhook_url: String,
    pub event_type: String,
    pub campaign_ids: Vec<u64>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhooksFilterDto {
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookPageDto {
    pub total_count: u32,
    pub items: Vec<WebhookDto>,
}
