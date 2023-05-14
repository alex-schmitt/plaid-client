/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ValidationSource : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct ValidationSource {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(
        rename = "ValidationSourceName",
        deserialize_with = "Option::deserialize"
    )]
    pub validation_source_name: Option<String>,
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(
        rename = "ValidationSourceReferenceIdentifier",
        deserialize_with = "Option::deserialize"
    )]
    pub validation_source_reference_identifier: Option<String>,
}

impl ValidationSource {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(
        validation_source_name: Option<String>,
        validation_source_reference_identifier: Option<String>,
    ) -> ValidationSource {
        ValidationSource {
            validation_source_name,
            validation_source_reference_identifier,
        }
    }
}
