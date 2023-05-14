/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferOriginatorCreateResponse : Defines the response schema for `/transfer/originator/create`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferOriginatorCreateResponse {
    /// Client ID of the originator. This identifier will be used when creating transfers and should be stored associated with end user information.
    #[serde(rename = "originator_client_id")]
    pub originator_client_id: String,
    /// The company name of the end customer.
    #[serde(rename = "company_name")]
    pub company_name: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferOriginatorCreateResponse {
    /// Defines the response schema for `/transfer/originator/create`
    pub fn new(
        originator_client_id: String,
        company_name: String,
        request_id: String,
    ) -> TransferOriginatorCreateResponse {
        TransferOriginatorCreateResponse {
            originator_client_id,
            company_name,
            request_id,
        }
    }
}