/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningIndividualReviewListResponse : Paginated list of screening reviews

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WatchlistScreeningIndividualReviewListResponse {
    /// List of screening reviews
    #[serde(rename = "watchlist_screening_reviews")]
    pub watchlist_screening_reviews: Vec<crate::models::WatchlistScreeningReview>,
    /// An identifier that determines which page of results you receive.
    #[serde(rename = "next_cursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl WatchlistScreeningIndividualReviewListResponse {
    /// Paginated list of screening reviews
    pub fn new(
        watchlist_screening_reviews: Vec<crate::models::WatchlistScreeningReview>,
        next_cursor: Option<String>,
        request_id: String,
    ) -> WatchlistScreeningIndividualReviewListResponse {
        WatchlistScreeningIndividualReviewListResponse {
            watchlist_screening_reviews,
            next_cursor,
            request_id,
        }
    }
}
