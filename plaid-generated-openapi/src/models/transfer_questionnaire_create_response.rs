/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferQuestionnaireCreateResponse : Defines the response schema for `/transfer/questionnaire/create`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferQuestionnaireCreateResponse {
    /// Plaid-hosted onboarding URL that you will redirect the end customer to.
    #[serde(rename = "onboarding_url")]
    pub onboarding_url: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferQuestionnaireCreateResponse {
    /// Defines the response schema for `/transfer/questionnaire/create`
    pub fn new(onboarding_url: String, request_id: String) -> TransferQuestionnaireCreateResponse {
        TransferQuestionnaireCreateResponse {
            onboarding_url,
            request_id,
        }
    }
}
