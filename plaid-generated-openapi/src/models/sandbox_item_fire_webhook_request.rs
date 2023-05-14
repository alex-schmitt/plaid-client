/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// SandboxItemFireWebhookRequest : SandboxItemFireWebhookRequest defines the request schema for `/sandbox/item/fire_webhook`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct SandboxItemFireWebhookRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "webhook_type", skip_serializing_if = "Option::is_none")]
    pub webhook_type: Option<crate::models::WebhookType>,
    /// The webhook codes that can be fired by this test endpoint.
    #[serde(rename = "webhook_code")]
    pub webhook_code: WebhookCode,
}

impl SandboxItemFireWebhookRequest {
    /// SandboxItemFireWebhookRequest defines the request schema for `/sandbox/item/fire_webhook`
    pub fn new(access_token: String, webhook_code: WebhookCode) -> SandboxItemFireWebhookRequest {
        SandboxItemFireWebhookRequest {
            client_id: None,
            secret: None,
            access_token,
            webhook_type: None,
            webhook_code,
        }
    }
}

/// The webhook codes that can be fired by this test endpoint.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
pub enum WebhookCode {
    #[serde(rename = "DEFAULT_UPDATE")]
    DefaultUpdate,
    #[serde(rename = "NEW_ACCOUNTS_AVAILABLE")]
    NewAccountsAvailable,
    #[serde(rename = "AUTH_DATA_UPDATE")]
    AuthDataUpdate,
    #[serde(rename = "RECURRING_TRANSACTIONS_UPDATE")]
    RecurringTransactionsUpdate,
    #[serde(rename = "SYNC_UPDATES_AVAILABLE")]
    SyncUpdatesAvailable,
}

impl Default for WebhookCode {
    fn default() -> WebhookCode {
        Self::DefaultUpdate
    }
}
