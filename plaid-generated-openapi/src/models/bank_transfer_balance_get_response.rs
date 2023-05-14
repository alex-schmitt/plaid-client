/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferBalanceGetResponse : Defines the response schema for `/bank_transfer/balance/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct BankTransferBalanceGetResponse {
    #[serde(rename = "balance")]
    pub balance: crate::models::BankTransferBalance,
    /// The ID of the origination account that this balance belongs to.
    #[serde(
        rename = "origination_account_id",
        deserialize_with = "Option::deserialize"
    )]
    pub origination_account_id: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BankTransferBalanceGetResponse {
    /// Defines the response schema for `/bank_transfer/balance/get`
    pub fn new(
        balance: crate::models::BankTransferBalance,
        origination_account_id: Option<String>,
        request_id: String,
    ) -> BankTransferBalanceGetResponse {
        BankTransferBalanceGetResponse {
            balance,
            origination_account_id,
            request_id,
        }
    }
}
