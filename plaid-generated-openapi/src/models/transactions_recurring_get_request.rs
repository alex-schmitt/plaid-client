/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRecurringGetRequest : TransactionsRecurringGetRequest defines the request schema for `/transactions/recurring/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransactionsRecurringGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::TransactionsRecurringGetRequestOptions>>,
    /// A list of `account_ids` to retrieve for the Item  Note: An error will be returned if a provided `account_id` is not associated with the Item.
    #[serde(rename = "account_ids")]
    pub account_ids: Vec<String>,
}

impl TransactionsRecurringGetRequest {
    /// TransactionsRecurringGetRequest defines the request schema for `/transactions/recurring/get`
    pub fn new(access_token: String, account_ids: Vec<String>) -> TransactionsRecurringGetRequest {
        TransactionsRecurringGetRequest {
            client_id: None,
            access_token,
            secret: None,
            options: None,
            account_ids,
        }
    }
}
