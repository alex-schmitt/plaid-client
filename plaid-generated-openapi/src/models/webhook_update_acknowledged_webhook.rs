/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WebhookUpdateAcknowledgedWebhook : Fired when an Item's webhook is updated. This will be sent to the newly specified webhook.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WebhookUpdateAcknowledgedWebhook {
    /// `ITEM`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `WEBHOOK_UPDATE_ACKNOWLEDGED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The new webhook URL
    #[serde(rename = "new_webhook_url")]
    pub new_webhook_url: String,
    #[serde(
        rename = "error",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<Option<crate::models::PlaidError>>,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl WebhookUpdateAcknowledgedWebhook {
    /// Fired when an Item's webhook is updated. This will be sent to the newly specified webhook.
    pub fn new(
        webhook_type: String,
        webhook_code: String,
        item_id: String,
        new_webhook_url: String,
        environment: crate::models::WebhookEnvironmentValues,
    ) -> WebhookUpdateAcknowledgedWebhook {
        WebhookUpdateAcknowledgedWebhook {
            webhook_type,
            webhook_code,
            item_id,
            new_webhook_url,
            error: None,
            environment,
        }
    }
}
