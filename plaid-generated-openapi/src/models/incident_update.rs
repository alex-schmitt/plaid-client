/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IncidentUpdate : An update on the health incident

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct IncidentUpdate {
    /// The content of the update.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The status of the incident.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The date when the update was published, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `\"2020-10-30T15:26:48Z\"`.
    #[serde(rename = "updated_date", skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<String>,
}

impl IncidentUpdate {
    /// An update on the health incident
    pub fn new() -> IncidentUpdate {
        IncidentUpdate {
            description: None,
            status: None,
            updated_date: None,
        }
    }
}

/// The status of the incident.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
pub enum Status {
    #[serde(rename = "INVESTIGATING")]
    Investigating,
    #[serde(rename = "IDENTIFIED")]
    Identified,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "RESOLVED")]
    Resolved,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for Status {
    fn default() -> Status {
        Self::Investigating
    }
}
