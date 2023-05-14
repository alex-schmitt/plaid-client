/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssetOwners : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct AssetOwners {
    /// Multiple Occurances of Account Owners Full Name up to 4.
    #[serde(rename = "ASSET_OWNER")]
    pub asset_owner: Vec<crate::models::AssetOwner>,
}

impl AssetOwners {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(asset_owner: Vec<crate::models::AssetOwner>) -> AssetOwners {
        AssetOwners { asset_owner }
    }
}
