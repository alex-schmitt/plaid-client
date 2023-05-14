/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PhysicalDocumentExtractedDataAnalysis : Analysis of the data extracted from the submitted document.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PhysicalDocumentExtractedDataAnalysis {
    #[serde(rename = "name")]
    pub name: crate::models::DocumentNameMatchCode,
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: crate::models::DocumentDateOfBirthMatchCode,
    #[serde(rename = "expiration_date")]
    pub expiration_date: crate::models::ExpirationDate,
    #[serde(rename = "issuing_country")]
    pub issuing_country: crate::models::IssuingCountry,
}

impl PhysicalDocumentExtractedDataAnalysis {
    /// Analysis of the data extracted from the submitted document.
    pub fn new(
        name: crate::models::DocumentNameMatchCode,
        date_of_birth: crate::models::DocumentDateOfBirthMatchCode,
        expiration_date: crate::models::ExpirationDate,
        issuing_country: crate::models::IssuingCountry,
    ) -> PhysicalDocumentExtractedDataAnalysis {
        PhysicalDocumentExtractedDataAnalysis {
            name,
            date_of_birth,
            expiration_date,
            issuing_country,
        }
    }
}
