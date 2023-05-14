/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartnerEndCustomerBillingContact : The billing contact for the end customer. Defaults to partner's billing contact if omitted.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PartnerEndCustomerBillingContact {
    #[serde(rename = "given_name", skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(rename = "family_name", skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl PartnerEndCustomerBillingContact {
    /// The billing contact for the end customer. Defaults to partner's billing contact if omitted.
    pub fn new() -> PartnerEndCustomerBillingContact {
        PartnerEndCustomerBillingContact {
            given_name: None,
            family_name: None,
            email: None,
        }
    }
}
