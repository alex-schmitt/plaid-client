/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacAssetsVoa24 : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditFreddieMacAssetsVoa24 {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET")]
    pub asset: Vec<crate::models::CreditFreddieMacAssetVoa24>,
}

impl CreditFreddieMacAssetsVoa24 {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(
        asset: Vec<crate::models::CreditFreddieMacAssetVoa24>,
    ) -> CreditFreddieMacAssetsVoa24 {
        CreditFreddieMacAssetsVoa24 { asset }
    }
}