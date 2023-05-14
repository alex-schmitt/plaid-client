/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityVerificationResponseUserName : The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct IdentityVerificationResponseUserName {
    /// A string with at least one non-whitespace character, with a max length of 100 characters.
    #[serde(rename = "given_name")]
    pub given_name: String,
    /// A string with at least one non-whitespace character, with a max length of 100 characters.
    #[serde(rename = "family_name")]
    pub family_name: String,
}

impl IdentityVerificationResponseUserName {
    /// The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.
    pub fn new(given_name: String, family_name: String) -> IdentityVerificationResponseUserName {
        IdentityVerificationResponseUserName {
            given_name,
            family_name,
        }
    }
}