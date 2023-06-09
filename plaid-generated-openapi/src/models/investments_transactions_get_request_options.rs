/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentsTransactionsGetRequestOptions : An optional object to filter `/investments/transactions/get` results. If provided, must be non-`null`.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct InvestmentsTransactionsGetRequestOptions {
    /// An array of `account_ids` to retrieve for the Item.
    #[serde(rename = "account_ids", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// The number of transactions to fetch.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The number of transactions to skip when fetching transaction history
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl InvestmentsTransactionsGetRequestOptions {
    /// An optional object to filter `/investments/transactions/get` results. If provided, must be non-`null`.
    pub fn new() -> InvestmentsTransactionsGetRequestOptions {
        InvestmentsTransactionsGetRequestOptions {
            account_ids: None,
            count: None,
            offset: None,
        }
    }
}
