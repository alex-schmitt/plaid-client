/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RecurringNewTransferWebhook : Fired when a new transfer of a recurring transfer is originated.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct RecurringNewTransferWebhook {
    /// `TRANSFER`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `RECURRING_NEW_TRANSFER`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// Plaid’s unique identifier for a recurring transfer.
    #[serde(rename = "recurring_transfer_id")]
    pub recurring_transfer_id: String,
    /// Plaid’s unique identifier for a transfer.
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl RecurringNewTransferWebhook {
    /// Fired when a new transfer of a recurring transfer is originated.
    pub fn new(
        webhook_type: String,
        webhook_code: String,
        recurring_transfer_id: String,
        transfer_id: String,
        environment: crate::models::WebhookEnvironmentValues,
    ) -> RecurringNewTransferWebhook {
        RecurringNewTransferWebhook {
            webhook_type,
            webhook_code,
            recurring_transfer_id,
            transfer_id,
            environment,
        }
    }
}
