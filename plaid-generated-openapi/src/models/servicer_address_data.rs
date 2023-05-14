/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ServicerAddressData : The address of the student loan servicer. This is generally the remittance address to which payments should be sent.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct ServicerAddressData {
    /// The full city name
    #[serde(rename = "city", deserialize_with = "Option::deserialize")]
    pub city: Option<String>,
    /// The region or state Example: `\"NC\"`
    #[serde(rename = "region", deserialize_with = "Option::deserialize")]
    pub region: Option<String>,
    /// The full street address Example: `\"564 Main Street, APT 15\"`
    #[serde(rename = "street", deserialize_with = "Option::deserialize")]
    pub street: Option<String>,
    /// The postal code
    #[serde(rename = "postal_code", deserialize_with = "Option::deserialize")]
    pub postal_code: Option<String>,
    /// The ISO 3166-1 alpha-2 country code
    #[serde(rename = "country", deserialize_with = "Option::deserialize")]
    pub country: Option<String>,
}

impl ServicerAddressData {
    /// The address of the student loan servicer. This is generally the remittance address to which payments should be sent.
    pub fn new(
        city: Option<String>,
        region: Option<String>,
        street: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
    ) -> ServicerAddressData {
        ServicerAddressData {
            city,
            region,
            street,
            postal_code,
            country,
        }
    }
}