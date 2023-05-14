/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditSessionResults : The set of results for a Link session.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditSessionResults {
    /// The set of Item adds for the Link session.
    #[serde(rename = "item_add_results", skip_serializing_if = "Option::is_none")]
    pub item_add_results: Option<Vec<crate::models::CreditSessionItemAddResult>>,
    /// The set of bank income verifications for the Link session.
    #[serde(
        rename = "bank_income_results",
        skip_serializing_if = "Option::is_none"
    )]
    pub bank_income_results: Option<Vec<crate::models::CreditSessionBankIncomeResult>>,
    /// The set of bank employment verifications for the Link session.
    #[serde(
        rename = "bank_employment_results",
        skip_serializing_if = "Option::is_none"
    )]
    pub bank_employment_results: Option<Vec<crate::models::CreditSessionBankEmploymentResult>>,
    /// The set of payroll income verifications for the Link session.
    #[serde(
        rename = "payroll_income_results",
        skip_serializing_if = "Option::is_none"
    )]
    pub payroll_income_results: Option<Vec<crate::models::CreditSessionPayrollIncomeResult>>,
    #[serde(
        rename = "document_income_results",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_income_results:
        Option<Option<Box<crate::models::CreditSessionDocumentIncomeResult>>>,
}

impl CreditSessionResults {
    /// The set of results for a Link session.
    pub fn new() -> CreditSessionResults {
        CreditSessionResults {
            item_add_results: None,
            bank_income_results: None,
            bank_employment_results: None,
            payroll_income_results: None,
            document_income_results: None,
        }
    }
}
