/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankEmploymentItem : The details and metadata for an end user's Item.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditBankEmploymentItem {
    /// The unique identifier for the Item.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The time when this Item's data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. \"2018-04-12T03:32:11Z\").
    #[serde(rename = "last_updated_time")]
    pub last_updated_time: String,
    /// The unique identifier of the institution associated with the Item.
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// The name of the institution associated with the Item.
    #[serde(rename = "institution_name")]
    pub institution_name: String,
    /// The bank employment information for this Item. Each entry in the array is a different employer found.
    #[serde(rename = "bank_employments")]
    pub bank_employments: Vec<crate::models::CreditBankEmployment>,
    /// The Item's accounts that have Bank Employment data.
    #[serde(rename = "bank_employment_accounts")]
    pub bank_employment_accounts: Vec<crate::models::CreditBankIncomeAccount>,
}

impl CreditBankEmploymentItem {
    /// The details and metadata for an end user's Item.
    pub fn new(
        item_id: String,
        last_updated_time: String,
        institution_id: String,
        institution_name: String,
        bank_employments: Vec<crate::models::CreditBankEmployment>,
        bank_employment_accounts: Vec<crate::models::CreditBankIncomeAccount>,
    ) -> CreditBankEmploymentItem {
        CreditBankEmploymentItem {
            item_id,
            last_updated_time,
            institution_id,
            institution_name,
            bank_employments,
            bank_employment_accounts,
        }
    }
}