/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferRefundCancelResponse : Defines the response schema for `/transfer/refund/cancel`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferRefundCancelResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferRefundCancelResponse {
    /// Defines the response schema for `/transfer/refund/cancel`
    pub fn new(request_id: String) -> TransferRefundCancelResponse {
        TransferRefundCancelResponse { request_id }
    }
}
