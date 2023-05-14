/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PhysicalDocumentExtractedData : Data extracted from a user-submitted document.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PhysicalDocumentExtractedData {
    /// Alpha-numeric ID number extracted via OCR from the user's document image.
    #[serde(rename = "id_number", deserialize_with = "Option::deserialize")]
    pub id_number: Option<String>,
    #[serde(rename = "category")]
    pub category: crate::models::PhysicalDocumentCategory,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "expiration_date", deserialize_with = "Option::deserialize")]
    pub expiration_date: Option<String>,
    /// Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(rename = "issuing_country")]
    pub issuing_country: String,
    /// An ISO 3166-2 subdivision code. Related terms would be \"state\", \"province\", \"prefecture\", \"zone\", \"subdivision\", etc.
    #[serde(rename = "issuing_region", deserialize_with = "Option::deserialize")]
    pub issuing_region: Option<String>,
}

impl PhysicalDocumentExtractedData {
    /// Data extracted from a user-submitted document.
    pub fn new(
        id_number: Option<String>,
        category: crate::models::PhysicalDocumentCategory,
        expiration_date: Option<String>,
        issuing_country: String,
        issuing_region: Option<String>,
    ) -> PhysicalDocumentExtractedData {
        PhysicalDocumentExtractedData {
            id_number,
            category,
            expiration_date,
            issuing_country,
            issuing_region,
        }
    }
}
