/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AccountsBalanceGetRequestOptions : An optional object to filter `/accounts/balance/get` results.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct AccountsBalanceGetRequestOptions {
    /// A list of `account_ids` to retrieve for the Item. The default value is `null`.  Note: An error will be returned if a provided `account_id` is not associated with the Item.
    #[serde(rename = "account_ids", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the oldest acceptable balance when making a request to `/accounts/balance/get`.  If the balance that is pulled for `ins_128026` (Capital One) is older than the given timestamp, an `INVALID_REQUEST` error with the code of `LAST_UPDATED_DATETIME_OUT_OF_RANGE` will be returned with the most recent timestamp for the requested account contained in the response.  This field is only used when the institution is `ins_128026` (Capital One), in which case a value must be provided or an `INVALID_REQUEST` error with the code of `INVALID_FIELD` will be returned. For all other institutions, this field is ignored.
    #[serde(
        rename = "min_last_updated_datetime",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_last_updated_datetime: Option<String>,
}

impl AccountsBalanceGetRequestOptions {
    /// An optional object to filter `/accounts/balance/get` results.
    pub fn new() -> AccountsBalanceGetRequestOptions {
        AccountsBalanceGetRequestOptions {
            account_ids: None,
            min_last_updated_datetime: None,
        }
    }
}
