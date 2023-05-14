/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// VerificationStatus : The verification status. One of the following:  `\"VERIFIED\"`: The information was successfully verified.  `\"UNVERIFIED\"`: The verification has not yet been performed.  `\"NEEDS_INFO\"`: The verification was attempted but could not be completed due to missing information.  \"`UNABLE_TO_VERIFY`\": The verification was performed and the information could not be verified.  `\"UNKNOWN\"`: The verification status is unknown.

/// The verification status. One of the following:  `\"VERIFIED\"`: The information was successfully verified.  `\"UNVERIFIED\"`: The verification has not yet been performed.  `\"NEEDS_INFO\"`: The verification was attempted but could not be completed due to missing information.  \"`UNABLE_TO_VERIFY`\": The verification was performed and the information could not be verified.  `\"UNKNOWN\"`: The verification status is unknown.
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
pub enum VerificationStatus {
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "UNVERIFIED")]
    Unverified,
    #[serde(rename = "NEEDS_INFO")]
    NeedsInfo,
    #[serde(rename = "UNABLE_TO_VERIFY")]
    UnableToVerify,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl ToString for VerificationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Verified => String::from("VERIFIED"),
            Self::Unverified => String::from("UNVERIFIED"),
            Self::NeedsInfo => String::from("NEEDS_INFO"),
            Self::UnableToVerify => String::from("UNABLE_TO_VERIFY"),
            Self::Unknown => String::from("UNKNOWN"),
        }
    }
}

impl Default for VerificationStatus {
    fn default() -> VerificationStatus {
        Self::Verified
    }
}
