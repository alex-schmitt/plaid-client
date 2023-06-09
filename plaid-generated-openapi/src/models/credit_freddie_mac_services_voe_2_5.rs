/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacServicesVoe25 : A collection of objects that describe requests and responses for services.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditFreddieMacServicesVoe25 {
    #[serde(rename = "SERVICE")]
    pub service: crate::models::CreditFreddieMacServiceVoe25,
}

impl CreditFreddieMacServicesVoe25 {
    /// A collection of objects that describe requests and responses for services.
    pub fn new(
        service: crate::models::CreditFreddieMacServiceVoe25,
    ) -> CreditFreddieMacServicesVoe25 {
        CreditFreddieMacServicesVoe25 { service }
    }
}
