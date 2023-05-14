/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScreeningHitDateOfBirthItem : Analyzed date of birth for the associated hit

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct ScreeningHitDateOfBirthItem {
    #[serde(rename = "analysis", skip_serializing_if = "Option::is_none")]
    pub analysis: Option<crate::models::MatchSummary>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::models::DateRange>,
}

impl ScreeningHitDateOfBirthItem {
    /// Analyzed date of birth for the associated hit
    pub fn new() -> ScreeningHitDateOfBirthItem {
        ScreeningHitDateOfBirthItem {
            analysis: None,
            data: None,
        }
    }
}
