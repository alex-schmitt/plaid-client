/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Address : A physical mailing address.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct Address {
    #[serde(rename = "data")]
    pub data: crate::models::AddressData,
    /// When `true`, identifies the address as the primary address on an account.
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

impl Address {
    /// A physical mailing address.
    pub fn new(data: crate::models::AddressData) -> Address {
        Address {
            data,
            primary: None,
        }
    }
}
