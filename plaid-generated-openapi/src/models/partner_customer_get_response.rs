/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartnerCustomerGetResponse : Response schema for `/partner/customer/get`.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PartnerCustomerGetResponse {
    #[serde(rename = "end_customer", skip_serializing_if = "Option::is_none")]
    pub end_customer: Option<crate::models::PartnerEndCustomer>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl PartnerCustomerGetResponse {
    /// Response schema for `/partner/customer/get`.
    pub fn new() -> PartnerCustomerGetResponse {
        PartnerCustomerGetResponse {
            end_customer: None,
            request_id: None,
        }
    }
}
