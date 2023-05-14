/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartnerCustomerEnableRequest : Request schema for `/partner/customer/enable`.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PartnerCustomerEnableRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "end_customer_client_id")]
    pub end_customer_client_id: String,
}

impl PartnerCustomerEnableRequest {
    /// Request schema for `/partner/customer/enable`.
    pub fn new(end_customer_client_id: String) -> PartnerCustomerEnableRequest {
        PartnerCustomerEnableRequest {
            client_id: None,
            secret: None,
            end_customer_client_id,
        }
    }
}