/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// NumbersInternationalIban : Account numbers using the International Bank Account Number and BIC/SWIFT code format.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct NumbersInternationalIban {
    /// International Bank Account Number (IBAN).
    #[serde(rename = "iban")]
    pub iban: String,
    /// The Business Identifier Code, also known as SWIFT code, for this bank account.
    #[serde(rename = "bic")]
    pub bic: String,
}

impl NumbersInternationalIban {
    /// Account numbers using the International Bank Account Number and BIC/SWIFT code format.
    pub fn new(iban: String, bic: String) -> NumbersInternationalIban {
        NumbersInternationalIban { iban, bic }
    }
}
