/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WebhookVerificationKeyGetResponse : WebhookVerificationKeyGetResponse defines the response schema for `/webhook_verification_key/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WebhookVerificationKeyGetResponse {
    #[serde(rename = "key")]
    pub key: crate::models::JwkPublicKey,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl WebhookVerificationKeyGetResponse {
    /// WebhookVerificationKeyGetResponse defines the response schema for `/webhook_verification_key/get`
    pub fn new(
        key: crate::models::JwkPublicKey,
        request_id: String,
    ) -> WebhookVerificationKeyGetResponse {
        WebhookVerificationKeyGetResponse { key, request_id }
    }
}