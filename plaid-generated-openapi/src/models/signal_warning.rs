/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// SignalWarning : Conveys information about the errors causing missing or stale bank data used to construct the /signal/evaluate scores and response

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct SignalWarning {
    /// Broad categorization of the warning.
    #[serde(rename = "warning_type", skip_serializing_if = "Option::is_none")]
    pub warning_type: Option<String>,
    /// The particular warning code.
    #[serde(rename = "warning_code", skip_serializing_if = "Option::is_none")]
    pub warning_code: Option<String>,
    /// A developer-friendly representation of the warning code.
    #[serde(rename = "warning_message", skip_serializing_if = "Option::is_none")]
    pub warning_message: Option<String>,
}

impl SignalWarning {
    /// Conveys information about the errors causing missing or stale bank data used to construct the /signal/evaluate scores and response
    pub fn new() -> SignalWarning {
        SignalWarning {
            warning_type: None,
            warning_code: None,
            warning_message: None,
        }
    }
}
