/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// EntityDocument : An official document, usually issued by a governing body or institution, with an associated identifier.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct EntityDocument {
    #[serde(rename = "type")]
    pub r#type: crate::models::EntityDocumentType,
    /// The numeric or alphanumeric identifier associated with this document.
    #[serde(rename = "number")]
    pub number: String,
}

impl EntityDocument {
    /// An official document, usually issued by a governing body or institution, with an associated identifier.
    pub fn new(r#type: crate::models::EntityDocumentType, number: String) -> EntityDocument {
        EntityDocument { r#type, number }
    }
}
