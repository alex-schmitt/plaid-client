/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WalletBalance : An object representing the e-wallet balance

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WalletBalance {
    /// The ISO-4217 currency code of the balance
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// The total amount of funds in the account
    #[serde(rename = "current")]
    pub current: f64,
}

impl WalletBalance {
    /// An object representing the e-wallet balance
    pub fn new(iso_currency_code: String, current: f64) -> WalletBalance {
        WalletBalance {
            iso_currency_code,
            current,
        }
    }
}
