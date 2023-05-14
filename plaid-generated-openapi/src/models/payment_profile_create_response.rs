/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaymentProfileCreateResponse : PaymentProfileCreateResponse defines the response schema for `/payment_profile/create`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PaymentProfileCreateResponse {
    /// A payment profile token associated with the Payment Profile data that is being requested.
    #[serde(rename = "payment_profile_token")]
    pub payment_profile_token: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentProfileCreateResponse {
    /// PaymentProfileCreateResponse defines the response schema for `/payment_profile/create`
    pub fn new(payment_profile_token: String, request_id: String) -> PaymentProfileCreateResponse {
        PaymentProfileCreateResponse {
            payment_profile_token,
            request_id,
        }
    }
}
