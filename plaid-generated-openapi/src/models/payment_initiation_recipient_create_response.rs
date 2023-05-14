/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationRecipientCreateResponse : PaymentInitiationRecipientCreateResponse defines the response schema for `/payment_initation/recipient/create`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PaymentInitiationRecipientCreateResponse {
    /// A unique ID identifying the recipient
    #[serde(rename = "recipient_id")]
    pub recipient_id: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentInitiationRecipientCreateResponse {
    /// PaymentInitiationRecipientCreateResponse defines the response schema for `/payment_initation/recipient/create`
    pub fn new(
        recipient_id: String,
        request_id: String,
    ) -> PaymentInitiationRecipientCreateResponse {
        PaymentInitiationRecipientCreateResponse {
            recipient_id,
            request_id,
        }
    }
}
