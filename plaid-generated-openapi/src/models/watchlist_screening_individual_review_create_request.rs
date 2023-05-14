/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningIndividualReviewCreateRequest : Request input for creating a screening review

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WatchlistScreeningIndividualReviewCreateRequest {
    /// Hits to mark as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    #[serde(rename = "confirmed_hits")]
    pub confirmed_hits: Vec<String>,
    /// Hits to mark as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    #[serde(rename = "dismissed_hits")]
    pub dismissed_hits: Vec<String>,
    /// A comment submitted by a team member as part of reviewing a watchlist screening.
    #[serde(
        rename = "comment",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub comment: Option<Option<String>>,
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// ID of the associated screening.
    #[serde(rename = "watchlist_screening_id")]
    pub watchlist_screening_id: String,
}

impl WatchlistScreeningIndividualReviewCreateRequest {
    /// Request input for creating a screening review
    pub fn new(
        confirmed_hits: Vec<String>,
        dismissed_hits: Vec<String>,
        watchlist_screening_id: String,
    ) -> WatchlistScreeningIndividualReviewCreateRequest {
        WatchlistScreeningIndividualReviewCreateRequest {
            confirmed_hits,
            dismissed_hits,
            comment: None,
            client_id: None,
            secret: None,
            watchlist_screening_id,
        }
    }
}
