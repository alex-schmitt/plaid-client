/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeRefreshRequestOptions : An optional object for `/credit/bank_income/refresh` request options.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditBankIncomeRefreshRequestOptions {
    /// How many days of data to include in the refresh. If not specified, this will default to the days requested in the most recently generated bank income report for the user.
    #[serde(rename = "days_requested", skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i32>,
    /// The URL where Plaid will send the bank income webhook.
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}

impl CreditBankIncomeRefreshRequestOptions {
    /// An optional object for `/credit/bank_income/refresh` request options.
    pub fn new() -> CreditBankIncomeRefreshRequestOptions {
        CreditBankIncomeRefreshRequestOptions {
            days_requested: None,
            webhook: None,
        }
    }
}
