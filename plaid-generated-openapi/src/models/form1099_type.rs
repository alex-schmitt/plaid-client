/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Form1099Type : Form 1099 Type

/// Form 1099 Type
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
pub enum Form1099Type {
    #[serde(rename = "FORM_1099_TYPE_UNKNOWN")]
    Unknown,
    #[serde(rename = "FORM_1099_TYPE_MISC")]
    Misc,
    #[serde(rename = "FORM_1099_TYPE_K")]
    K,
}

impl ToString for Form1099Type {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("FORM_1099_TYPE_UNKNOWN"),
            Self::Misc => String::from("FORM_1099_TYPE_MISC"),
            Self::K => String::from("FORM_1099_TYPE_K"),
        }
    }
}

impl Default for Form1099Type {
    fn default() -> Form1099Type {
        Self::Unknown
    }
}
