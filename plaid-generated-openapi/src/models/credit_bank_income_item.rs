/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeItem : The details and metadata for an end user's Item.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditBankIncomeItem {
    /// The Item's accounts that have Bank Income data.
    #[serde(
        rename = "bank_income_accounts",
        skip_serializing_if = "Option::is_none"
    )]
    pub bank_income_accounts: Option<Vec<crate::models::CreditBankIncomeAccount>>,
    /// The income sources for this Item. Each entry in the array is a single income source.
    #[serde(
        rename = "bank_income_sources",
        skip_serializing_if = "Option::is_none"
    )]
    pub bank_income_sources: Option<Vec<crate::models::CreditBankIncomeSource>>,
    /// The time when this Item's data was last retrieved from the financial institution.
    #[serde(rename = "last_updated_time", skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// The unique identifier of the institution associated with the Item.
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    /// The name of the institution associated with the Item.
    #[serde(rename = "institution_name", skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// The unique identifier for the Item.
    #[serde(rename = "item_id", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

impl CreditBankIncomeItem {
    /// The details and metadata for an end user's Item.
    pub fn new() -> CreditBankIncomeItem {
        CreditBankIncomeItem {
            bank_income_accounts: None,
            bank_income_sources: None,
            last_updated_time: None,
            institution_id: None,
            institution_name: None,
            item_id: None,
        }
    }
}
