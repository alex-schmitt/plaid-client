/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// SimulatedTransferSweep : A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint. Can be null if there are no transfers to include in a sweep.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct SimulatedTransferSweep {
    /// Identifier of the sweep.
    #[serde(rename = "id")]
    pub id: String,
    /// The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited.
    #[serde(rename = "funding_account_id")]
    pub funding_account_id: String,
    /// The datetime when the sweep occurred, in RFC 3339 format.
    #[serde(rename = "created")]
    pub created: String,
    /// Signed decimal amount of the sweep as it appears on your sweep account ledger (e.g. \"-10.00\")  If amount is not present, the sweep was net-settled to zero and outstanding debits and credits between the sweep account and Plaid are balanced.
    #[serde(rename = "amount")]
    pub amount: String,
    /// The currency of the sweep, e.g. \"USD\".
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// The date when the sweep settled, in the YYYY-MM-DD format.
    #[serde(rename = "settled", deserialize_with = "Option::deserialize")]
    pub settled: Option<String>,
}

impl SimulatedTransferSweep {
    /// A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint. Can be null if there are no transfers to include in a sweep.
    pub fn new(
        id: String,
        funding_account_id: String,
        created: String,
        amount: String,
        iso_currency_code: String,
        settled: Option<String>,
    ) -> SimulatedTransferSweep {
        SimulatedTransferSweep {
            id,
            funding_account_id,
            created,
            amount,
            iso_currency_code,
            settled,
        }
    }
}
