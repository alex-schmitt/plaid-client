/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityUpdateTypes : The possible types of identity data that may have changed.

/// The possible types of identity data that may have changed.
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
pub enum IdentityUpdateTypes {
    #[serde(rename = "PHONES")]
    Phones,
    #[serde(rename = "ADDRESSES")]
    Addresses,
    #[serde(rename = "EMAILS")]
    Emails,
    #[serde(rename = "NAMES")]
    Names,
}

impl ToString for IdentityUpdateTypes {
    fn to_string(&self) -> String {
        match self {
            Self::Phones => String::from("PHONES"),
            Self::Addresses => String::from("ADDRESSES"),
            Self::Emails => String::from("EMAILS"),
            Self::Names => String::from("NAMES"),
        }
    }
}

impl Default for IdentityUpdateTypes {
    fn default() -> IdentityUpdateTypes {
        Self::Phones
    }
}
