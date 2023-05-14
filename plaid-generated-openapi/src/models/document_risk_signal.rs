/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentRiskSignal : Details about a certain reason as to why a document could potentially be fraudulent.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct DocumentRiskSignal {
    /// The result from the risk signal check.
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<String>,
    /// The field which the risk signal was computed for
    #[serde(rename = "field", deserialize_with = "Option::deserialize")]
    pub field: Option<String>,
    /// A flag used to quickly identify if the signal indicates that this field is authentic or fraudulent
    #[serde(rename = "has_fraud_risk", deserialize_with = "Option::deserialize")]
    pub has_fraud_risk: Option<bool>,
    #[serde(
        rename = "institution_metadata",
        deserialize_with = "Option::deserialize"
    )]
    pub institution_metadata: Option<crate::models::DocumentRiskSignalInstitutionMetadata>,
    /// The expected value of the field, as seen on the document
    #[serde(rename = "expected_value", deserialize_with = "Option::deserialize")]
    pub expected_value: Option<String>,
    /// The derived value obtained in the risk signal calculation process for this field
    #[serde(rename = "actual_value", deserialize_with = "Option::deserialize")]
    pub actual_value: Option<String>,
    /// A human-readable explanation providing more detail into the particular risk signal
    #[serde(
        rename = "signal_description",
        deserialize_with = "Option::deserialize"
    )]
    pub signal_description: Option<String>,
    /// The relevant page associated with the risk signal
    #[serde(rename = "page_number", deserialize_with = "Option::deserialize")]
    pub page_number: Option<i32>,
}

impl DocumentRiskSignal {
    /// Details about a certain reason as to why a document could potentially be fraudulent.
    pub fn new(
        r#type: Option<String>,
        field: Option<String>,
        has_fraud_risk: Option<bool>,
        institution_metadata: Option<crate::models::DocumentRiskSignalInstitutionMetadata>,
        expected_value: Option<String>,
        actual_value: Option<String>,
        signal_description: Option<String>,
        page_number: Option<i32>,
    ) -> DocumentRiskSignal {
        DocumentRiskSignal {
            r#type,
            field,
            has_fraud_risk,
            institution_metadata,
            expected_value,
            actual_value,
            signal_description,
            page_number,
        }
    }
}
