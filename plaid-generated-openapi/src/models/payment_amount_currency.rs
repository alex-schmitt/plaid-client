/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaymentAmountCurrency : The ISO-4217 currency code of the payment. For standing orders and payment consents, `\"GBP\"` must be used.

/// The ISO-4217 currency code of the payment. For standing orders and payment consents, `\"GBP\"` must be used.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
pub enum PaymentAmountCurrency {
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "NOK")]
    Nok,
}

impl ToString for PaymentAmountCurrency {
    fn to_string(&self) -> String {
        match self {
            Self::Gbp => String::from("GBP"),
            Self::Eur => String::from("EUR"),
            Self::Pln => String::from("PLN"),
            Self::Sek => String::from("SEK"),
            Self::Dkk => String::from("DKK"),
            Self::Nok => String::from("NOK"),
        }
    }
}

impl Default for PaymentAmountCurrency {
    fn default() -> PaymentAmountCurrency {
        Self::Gbp
    }
}
