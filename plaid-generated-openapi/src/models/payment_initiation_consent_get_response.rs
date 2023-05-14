/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationConsentGetResponse : PaymentInitiationConsentGetResponse defines the response schema for `/payment_initation/consent/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PaymentInitiationConsentGetResponse {
    /// The consent ID.
    #[serde(rename = "consent_id")]
    pub consent_id: String,
    #[serde(rename = "status")]
    pub status: crate::models::PaymentInitiationConsentStatus,
    /// Consent creation timestamp, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The ID of the recipient the payment consent is for.
    #[serde(rename = "recipient_id")]
    pub recipient_id: String,
    /// A reference for the payment consent.
    #[serde(rename = "reference")]
    pub reference: String,
    #[serde(rename = "constraints")]
    pub constraints: crate::models::PaymentInitiationConsentConstraints,
    /// An array of payment consent scopes.
    #[serde(rename = "scopes")]
    pub scopes: Vec<crate::models::PaymentInitiationConsentScope>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentInitiationConsentGetResponse {
    /// PaymentInitiationConsentGetResponse defines the response schema for `/payment_initation/consent/get`
    pub fn new(
        consent_id: String,
        status: crate::models::PaymentInitiationConsentStatus,
        created_at: String,
        recipient_id: String,
        reference: String,
        constraints: crate::models::PaymentInitiationConsentConstraints,
        scopes: Vec<crate::models::PaymentInitiationConsentScope>,
        request_id: String,
    ) -> PaymentInitiationConsentGetResponse {
        PaymentInitiationConsentGetResponse {
            consent_id,
            status,
            created_at,
            recipient_id,
            reference,
            constraints,
            scopes,
            request_id,
        }
    }
}
