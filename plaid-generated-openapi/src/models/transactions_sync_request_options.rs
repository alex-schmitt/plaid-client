/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsSyncRequestOptions : An optional object to be used with the request. If specified, `options` must not be `null`.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransactionsSyncRequestOptions {
    /// Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager.
    #[serde(
        rename = "include_original_description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_original_description: Option<Option<bool>>,
    /// Include the [`personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-sync-response-added-personal-finance-category) object in the response.  See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.  We’re introducing Category Rules - a new beta endpoint that will enable you to change the `personal_finance_category` for a transaction based on your users’ needs. When rules are set, the selected category will override the Plaid provided category. To learn more, send a note to transactions-feedback@plaid.com.
    #[serde(
        rename = "include_personal_finance_category",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_personal_finance_category: Option<bool>,
    /// Include counterparties and extra merchant fields in the transaction. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager.
    #[serde(
        rename = "include_logo_and_counterparty_beta",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_logo_and_counterparty_beta: Option<bool>,
}

impl TransactionsSyncRequestOptions {
    /// An optional object to be used with the request. If specified, `options` must not be `null`.
    pub fn new() -> TransactionsSyncRequestOptions {
        TransactionsSyncRequestOptions {
            include_original_description: None,
            include_personal_finance_category: None,
            include_logo_and_counterparty_beta: None,
        }
    }
}
