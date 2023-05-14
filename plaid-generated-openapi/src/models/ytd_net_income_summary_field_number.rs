/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// YtdNetIncomeSummaryFieldNumber : Year-to-date earnings after any tax withholdings, benefit payments or deductions, as reported on the paystub.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct YtdNetIncomeSummaryFieldNumber {
    /// The value of the field.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "verification_status")]
    pub verification_status: crate::models::VerificationStatus,
}

impl YtdNetIncomeSummaryFieldNumber {
    /// Year-to-date earnings after any tax withholdings, benefit payments or deductions, as reported on the paystub.
    pub fn new(
        value: f64,
        verification_status: crate::models::VerificationStatus,
    ) -> YtdNetIncomeSummaryFieldNumber {
        YtdNetIncomeSummaryFieldNumber {
            value,
            verification_status,
        }
    }
}
