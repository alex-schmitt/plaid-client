/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// EntityWatchlistProgram : A program that configures the active lists, search parameters, and other behavior for initial and ongoing screening of entities.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct EntityWatchlistProgram {
    /// ID of the associated entity program.
    #[serde(rename = "id")]
    pub id: String,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Indicator specifying whether the program is enabled and will perform daily rescans.
    #[serde(rename = "is_rescanning_enabled")]
    pub is_rescanning_enabled: bool,
    /// Watchlists enabled for the associated program
    #[serde(rename = "lists_enabled")]
    pub lists_enabled: Vec<crate::models::EntityWatchlistCode>,
    /// A name for the entity program to define its purpose. For example, \"High Risk Organizations\" or \"Applicants\".
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "name_sensitivity")]
    pub name_sensitivity: crate::models::ProgramNameSensitivity,
    #[serde(rename = "audit_trail")]
    pub audit_trail: crate::models::WatchlistScreeningAuditTrail,
    /// Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    #[serde(rename = "is_archived")]
    pub is_archived: bool,
}

impl EntityWatchlistProgram {
    /// A program that configures the active lists, search parameters, and other behavior for initial and ongoing screening of entities.
    pub fn new(
        id: String,
        created_at: String,
        is_rescanning_enabled: bool,
        lists_enabled: Vec<crate::models::EntityWatchlistCode>,
        name: String,
        name_sensitivity: crate::models::ProgramNameSensitivity,
        audit_trail: crate::models::WatchlistScreeningAuditTrail,
        is_archived: bool,
    ) -> EntityWatchlistProgram {
        EntityWatchlistProgram {
            id,
            created_at,
            is_rescanning_enabled,
            lists_enabled,
            name,
            name_sensitivity,
            audit_trail,
            is_archived,
        }
    }
}
