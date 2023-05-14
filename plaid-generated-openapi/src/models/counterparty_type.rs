/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CounterpartyType : The counterparty type.  `merchant`: a provider of goods or services for purchase `financial_institution`: a financial entity (bank, credit union, BNPL, fintech) `payment_app`: a transfer or P2P app (e.g. Zelle) `marketplace`: a marketplace (e.g DoorDash, Google Play Store) `payment_terminal`: a point-of-sale payment terminal (e.g Square, Toast)

/// The counterparty type.  `merchant`: a provider of goods or services for purchase `financial_institution`: a financial entity (bank, credit union, BNPL, fintech) `payment_app`: a transfer or P2P app (e.g. Zelle) `marketplace`: a marketplace (e.g DoorDash, Google Play Store) `payment_terminal`: a point-of-sale payment terminal (e.g Square, Toast)
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
pub enum CounterpartyType {
    #[serde(rename = "merchant")]
    Merchant,
    #[serde(rename = "financial_institution")]
    FinancialInstitution,
    #[serde(rename = "payment_app")]
    PaymentApp,
    #[serde(rename = "marketplace")]
    Marketplace,
    #[serde(rename = "payment_terminal")]
    PaymentTerminal,
}

impl ToString for CounterpartyType {
    fn to_string(&self) -> String {
        match self {
            Self::Merchant => String::from("merchant"),
            Self::FinancialInstitution => String::from("financial_institution"),
            Self::PaymentApp => String::from("payment_app"),
            Self::Marketplace => String::from("marketplace"),
            Self::PaymentTerminal => String::from("payment_terminal"),
        }
    }
}

impl Default for CounterpartyType {
    fn default() -> CounterpartyType {
        Self::Merchant
    }
}