/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LoanIdentifiers : Collection of current and previous identifiers for this loan.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct LoanIdentifiers {
    #[serde(rename = "LOAN_IDENTIFIER")]
    pub loan_identifier: crate::models::LoanIdentifier,
}

impl LoanIdentifiers {
    /// Collection of current and previous identifiers for this loan.
    pub fn new(loan_identifier: crate::models::LoanIdentifier) -> LoanIdentifiers {
        LoanIdentifiers { loan_identifier }
    }
}