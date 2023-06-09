/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AccountsGetResponse : AccountsGetResponse defines the response schema for `/accounts/get` and `/accounts/balance/get`.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct AccountsGetResponse {
    /// An array of financial institution accounts associated with the Item. If `/accounts/balance/get` was called, each account will include real-time balance information.
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::AccountBase>,
    #[serde(rename = "item")]
    pub item: crate::models::Item,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl AccountsGetResponse {
    /// AccountsGetResponse defines the response schema for `/accounts/get` and `/accounts/balance/get`.
    pub fn new(
        accounts: Vec<crate::models::AccountBase>,
        item: crate::models::Item,
        request_id: String,
    ) -> AccountsGetResponse {
        AccountsGetResponse {
            accounts,
            item,
            request_id,
        }
    }
}
