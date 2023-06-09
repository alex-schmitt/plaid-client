/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FreddieReportType : When set to `VERIFICATION_OF_EMPLOYMENT` and the Asset Report is added to an Audit Copy Token, the Asset Report will be retrieved by Freddie Mac in the Verification Of Employment (VOE) version instead of the default Verification Of Assets (VOA) version.

/// When set to `VERIFICATION_OF_EMPLOYMENT` and the Asset Report is added to an Audit Copy Token, the Asset Report will be retrieved by Freddie Mac in the Verification Of Employment (VOE) version instead of the default Verification Of Assets (VOA) version.
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
pub enum FreddieReportType {
    #[serde(rename = "VERIFICATION_OF_EMPLOYMENT")]
    VerificationOfEmployment,
}

impl ToString for FreddieReportType {
    fn to_string(&self) -> String {
        match self {
            Self::VerificationOfEmployment => String::from("VERIFICATION_OF_EMPLOYMENT"),
        }
    }
}

impl Default for FreddieReportType {
    fn default() -> FreddieReportType {
        Self::VerificationOfEmployment
    }
}
