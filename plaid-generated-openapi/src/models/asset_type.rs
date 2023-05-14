/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssetType : A value from a MISMO prescribed list that specifies financial assets in a mortgage loan transaction. Assets may be either liquid or fixed and are associated with a corresponding asset amount.

/// A value from a MISMO prescribed list that specifies financial assets in a mortgage loan transaction. Assets may be either liquid or fixed and are associated with a corresponding asset amount.
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
pub enum AssetType {
    #[serde(rename = "CheckingAccount")]
    CheckingAccount,
    #[serde(rename = "SavingsAccount")]
    SavingsAccount,
    #[serde(rename = "Investment")]
    Investment,
    #[serde(rename = "MoneyMarketFund")]
    MoneyMarketFund,
    #[serde(rename = "Other")]
    Other,
}

impl ToString for AssetType {
    fn to_string(&self) -> String {
        match self {
            Self::CheckingAccount => String::from("CheckingAccount"),
            Self::SavingsAccount => String::from("SavingsAccount"),
            Self::Investment => String::from("Investment"),
            Self::MoneyMarketFund => String::from("MoneyMarketFund"),
            Self::Other => String::from("Other"),
        }
    }
}

impl Default for AssetType {
    fn default() -> AssetType {
        Self::CheckingAccount
    }
}
