/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaymentConsentPeriodicAmountAmount : Maximum cumulative amount for all payments in the specified interval.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PaymentConsentPeriodicAmountAmount {
    #[serde(rename = "currency")]
    pub currency: crate::models::PaymentAmountCurrency,
    /// The amount of the payment. Must contain at most two digits of precision e.g. `1.23`. Minimum accepted value is `1`.
    #[serde(rename = "value")]
    pub value: f64,
}

impl PaymentConsentPeriodicAmountAmount {
    /// Maximum cumulative amount for all payments in the specified interval.
    pub fn new(
        currency: crate::models::PaymentAmountCurrency,
        value: f64,
    ) -> PaymentConsentPeriodicAmountAmount {
        PaymentConsentPeriodicAmountAmount { currency, value }
    }
}