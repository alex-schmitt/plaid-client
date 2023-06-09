/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsEnrichGetResponse : TransactionsEnrichGetResponse defines the response schema for `/transactions/enrich`.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransactionsEnrichGetResponse {
    /// A list of enriched transactions.
    #[serde(rename = "enriched_transactions")]
    pub enriched_transactions: Vec<crate::models::ClientProvidedEnrichedTransaction>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl TransactionsEnrichGetResponse {
    /// TransactionsEnrichGetResponse defines the response schema for `/transactions/enrich`.
    pub fn new(
        enriched_transactions: Vec<crate::models::ClientProvidedEnrichedTransaction>,
    ) -> TransactionsEnrichGetResponse {
        TransactionsEnrichGetResponse {
            enriched_transactions,
            request_id: None,
        }
    }
}
