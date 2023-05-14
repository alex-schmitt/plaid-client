/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportAddOns : A list of add-ons that should be included in the Asset Report

/// A list of add-ons that should be included in the Asset Report
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
pub enum AssetReportAddOns {
    #[serde(rename = "investments")]
    Investments,
    #[serde(rename = "fast_assets")]
    FastAssets,
}

impl ToString for AssetReportAddOns {
    fn to_string(&self) -> String {
        match self {
            Self::Investments => String::from("investments"),
            Self::FastAssets => String::from("fast_assets"),
        }
    }
}

impl Default for AssetReportAddOns {
    fn default() -> AssetReportAddOns {
        Self::Investments
    }
}