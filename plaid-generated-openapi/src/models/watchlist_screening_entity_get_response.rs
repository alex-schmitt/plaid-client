/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningEntityGetResponse : The entity screening object allows you to represent an entity in your system, update its profile, and search for it on various watchlists. Note: Rejected entity screenings will not receive new hits, regardless of entity program configuration.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WatchlistScreeningEntityGetResponse {
    /// ID of the associated entity screening.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "search_terms")]
    pub search_terms: crate::models::EntityWatchlistScreeningSearchTerms,
    /// ID of the associated user.
    #[serde(rename = "assignee", deserialize_with = "Option::deserialize")]
    pub assignee: Option<String>,
    #[serde(rename = "status")]
    pub status: crate::models::WatchlistScreeningStatus,
    /// An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    #[serde(rename = "client_user_id", deserialize_with = "Option::deserialize")]
    pub client_user_id: Option<String>,
    #[serde(rename = "audit_trail")]
    pub audit_trail: crate::models::WatchlistScreeningAuditTrail,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl WatchlistScreeningEntityGetResponse {
    /// The entity screening object allows you to represent an entity in your system, update its profile, and search for it on various watchlists. Note: Rejected entity screenings will not receive new hits, regardless of entity program configuration.
    pub fn new(
        id: String,
        search_terms: crate::models::EntityWatchlistScreeningSearchTerms,
        assignee: Option<String>,
        status: crate::models::WatchlistScreeningStatus,
        client_user_id: Option<String>,
        audit_trail: crate::models::WatchlistScreeningAuditTrail,
        request_id: String,
    ) -> WatchlistScreeningEntityGetResponse {
        WatchlistScreeningEntityGetResponse {
            id,
            search_terms,
            assignee,
            status,
            client_user_id,
            audit_trail,
            request_id,
        }
    }
}
