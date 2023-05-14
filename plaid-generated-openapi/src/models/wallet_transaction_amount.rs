/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WalletTransactionAmount : The amount and currency of a transaction

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WalletTransactionAmount {
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: crate::models::WalletIsoCurrencyCode,
    /// The amount of the transaction. Must contain at most two digits of precision e.g. `1.23`.
    #[serde(rename = "value")]
    pub value: f64,
}

impl WalletTransactionAmount {
    /// The amount and currency of a transaction
    pub fn new(
        iso_currency_code: crate::models::WalletIsoCurrencyCode,
        value: f64,
    ) -> WalletTransactionAmount {
        WalletTransactionAmount {
            iso_currency_code,
            value,
        }
    }
}
