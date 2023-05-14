/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WalletTransactionsListRequest : WalletTransactionListRequest defines the request schema for `/wallet/transaction/list`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WalletTransactionsListRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the e-wallet to fetch transactions from
    #[serde(rename = "wallet_id")]
    pub wallet_id: String,
    /// A base64 value representing the latest transaction that has already been requested. Set this to `next_cursor` received from the previous `/wallet/transaction/list` request. If provided, the response will only contain transactions created before that transaction. If omitted, the response will contain transactions starting from the most recent, and in descending order by the `created_at` time.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of transactions to fetch
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        rename = "options",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub options: Option<Option<Box<crate::models::WalletTransactionListRequestOptions>>>,
}

impl WalletTransactionsListRequest {
    /// WalletTransactionListRequest defines the request schema for `/wallet/transaction/list`
    pub fn new(wallet_id: String) -> WalletTransactionsListRequest {
        WalletTransactionsListRequest {
            client_id: None,
            secret: None,
            wallet_id,
            cursor: None,
            count: None,
            options: None,
        }
    }
}
