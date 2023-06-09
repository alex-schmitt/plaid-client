/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacServiceVoe25 : A collection of details related to a fulfillment service or product in terms of request, process and result.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditFreddieMacServiceVoe25 {
    #[serde(rename = "VERIFICATION_OF_ASSET")]
    pub verification_of_asset: Vec<crate::models::CreditFreddieMacVerificationOfAssetVoe25>,
    #[serde(rename = "STATUSES")]
    pub statuses: crate::models::Statuses,
}

impl CreditFreddieMacServiceVoe25 {
    /// A collection of details related to a fulfillment service or product in terms of request, process and result.
    pub fn new(
        verification_of_asset: Vec<crate::models::CreditFreddieMacVerificationOfAssetVoe25>,
        statuses: crate::models::Statuses,
    ) -> CreditFreddieMacServiceVoe25 {
        CreditFreddieMacServiceVoe25 {
            verification_of_asset,
            statuses,
        }
    }
}
