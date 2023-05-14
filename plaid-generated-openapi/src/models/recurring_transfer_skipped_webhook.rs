/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RecurringTransferSkippedWebhook : Fired when Plaid is unable to originate a new ACH transaction of the recurring transfer on the planned date.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct RecurringTransferSkippedWebhook {
    /// `TRANSFER`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `RECURRING_TRANSFER_SKIPPED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// Plaid’s unique identifier for a recurring transfer.
    #[serde(rename = "recurring_transfer_id")]
    pub recurring_transfer_id: String,
    #[serde(rename = "authorization_decision")]
    pub authorization_decision: crate::models::TransferAuthorizationDecision,
    #[serde(
        rename = "authorization_decision_rationale_code",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_decision_rationale_code:
        Option<crate::models::TransferAuthorizationDecisionRationaleCode>,
    /// The planned date on which Plaid is unable to originate a new ACH transaction of the recurring transfer. This will be of the form YYYY-MM-DD.
    #[serde(rename = "skipped_origination_date")]
    pub skipped_origination_date: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl RecurringTransferSkippedWebhook {
    /// Fired when Plaid is unable to originate a new ACH transaction of the recurring transfer on the planned date.
    pub fn new(
        webhook_type: String,
        webhook_code: String,
        recurring_transfer_id: String,
        authorization_decision: crate::models::TransferAuthorizationDecision,
        skipped_origination_date: String,
        environment: crate::models::WebhookEnvironmentValues,
    ) -> RecurringTransferSkippedWebhook {
        RecurringTransferSkippedWebhook {
            webhook_type,
            webhook_code,
            recurring_transfer_id,
            authorization_decision,
            authorization_decision_rationale_code: None,
            skipped_origination_date,
            environment,
        }
    }
}
