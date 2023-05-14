/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferEvent : Represents an event in the Bank Transfers API.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct BankTransferEvent {
    /// Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    #[serde(rename = "event_id")]
    pub event_id: i32,
    /// The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "event_type")]
    pub event_type: crate::models::BankTransferEventType,
    /// The account ID associated with the bank transfer.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Plaid’s unique identifier for a bank transfer.
    #[serde(rename = "bank_transfer_id")]
    pub bank_transfer_id: String,
    /// The ID of the origination account that this balance belongs to.
    #[serde(
        rename = "origination_account_id",
        deserialize_with = "Option::deserialize"
    )]
    pub origination_account_id: Option<String>,
    #[serde(rename = "bank_transfer_type")]
    pub bank_transfer_type: crate::models::BankTransferType,
    /// The bank transfer amount.
    #[serde(rename = "bank_transfer_amount")]
    pub bank_transfer_amount: String,
    /// The currency of the bank transfer amount.
    #[serde(rename = "bank_transfer_iso_currency_code")]
    pub bank_transfer_iso_currency_code: String,
    #[serde(rename = "failure_reason", deserialize_with = "Option::deserialize")]
    pub failure_reason: Option<crate::models::BankTransferFailure>,
    #[serde(rename = "direction", deserialize_with = "Option::deserialize")]
    pub direction: Option<crate::models::BankTransferDirection>,
}

impl BankTransferEvent {
    /// Represents an event in the Bank Transfers API.
    pub fn new(
        event_id: i32,
        timestamp: String,
        event_type: crate::models::BankTransferEventType,
        account_id: String,
        bank_transfer_id: String,
        origination_account_id: Option<String>,
        bank_transfer_type: crate::models::BankTransferType,
        bank_transfer_amount: String,
        bank_transfer_iso_currency_code: String,
        failure_reason: Option<crate::models::BankTransferFailure>,
        direction: Option<crate::models::BankTransferDirection>,
    ) -> BankTransferEvent {
        BankTransferEvent {
            event_id,
            timestamp,
            event_type,
            account_id,
            bank_transfer_id,
            origination_account_id,
            bank_transfer_type,
            bank_transfer_amount,
            bank_transfer_iso_currency_code,
            failure_reason,
            direction,
        }
    }
}