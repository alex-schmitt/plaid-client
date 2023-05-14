/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditRelayRemoveResponse : CreditRelayRemoveResponse defines the response schema for `/credit/relay/remove`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditRelayRemoveResponse {
    /// `true` if the relay token was successfully removed.
    #[serde(rename = "removed")]
    pub removed: bool,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CreditRelayRemoveResponse {
    /// CreditRelayRemoveResponse defines the response schema for `/credit/relay/remove`
    pub fn new(removed: bool, request_id: String) -> CreditRelayRemoveResponse {
        CreditRelayRemoveResponse {
            removed,
            request_id,
        }
    }
}