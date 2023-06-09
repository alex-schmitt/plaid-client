/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartnerEndCustomer : The details for an end customer.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PartnerEndCustomer {
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::PartnerEndCustomerStatus>,
}

impl PartnerEndCustomer {
    /// The details for an end customer.
    pub fn new() -> PartnerEndCustomer {
        PartnerEndCustomer {
            client_id: None,
            company_name: None,
            status: None,
        }
    }
}
