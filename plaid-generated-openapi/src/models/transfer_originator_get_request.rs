/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferOriginatorGetRequest : Defines the request schema for `/transfer/originator/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferOriginatorGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Client ID of the end customer (i.e. the originator).
    #[serde(rename = "originator_client_id")]
    pub originator_client_id: String,
}

impl TransferOriginatorGetRequest {
    /// Defines the request schema for `/transfer/originator/get`
    pub fn new(originator_client_id: String) -> TransferOriginatorGetRequest {
        TransferOriginatorGetRequest {
            client_id: None,
            secret: None,
            originator_client_id,
        }
    }
}
