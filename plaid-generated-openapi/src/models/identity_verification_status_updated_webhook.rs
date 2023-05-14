/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityVerificationStatusUpdatedWebhook : Fired when the status of an identity verification has been updated, which can be triggered via the dashboard or the API.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct IdentityVerificationStatusUpdatedWebhook {
    /// `IDENTITY_VERIFICATION`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `STATUS_UPDATED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The ID of the associated Identity Verification attempt.
    #[serde(rename = "identity_verification_id")]
    pub identity_verification_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl IdentityVerificationStatusUpdatedWebhook {
    /// Fired when the status of an identity verification has been updated, which can be triggered via the dashboard or the API.
    pub fn new(
        webhook_type: String,
        webhook_code: String,
        identity_verification_id: String,
        environment: crate::models::WebhookEnvironmentValues,
    ) -> IdentityVerificationStatusUpdatedWebhook {
        IdentityVerificationStatusUpdatedWebhook {
            webhook_type,
            webhook_code,
            identity_verification_id,
            environment,
        }
    }
}
