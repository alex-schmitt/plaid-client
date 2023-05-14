/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferSweepGetResponse : Defines the response schema for `/transfer/sweep/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferSweepGetResponse {
    #[serde(rename = "sweep")]
    pub sweep: crate::models::TransferSweep,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferSweepGetResponse {
    /// Defines the response schema for `/transfer/sweep/get`
    pub fn new(
        sweep: crate::models::TransferSweep,
        request_id: String,
    ) -> TransferSweepGetResponse {
        TransferSweepGetResponse { sweep, request_id }
    }
}
