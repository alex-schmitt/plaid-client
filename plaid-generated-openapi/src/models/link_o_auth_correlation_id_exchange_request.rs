/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LinkOAuthCorrelationIdExchangeRequest : LinkOAuthCorrelationIdExchangeRequest defines the request schema for `/link/oauth/correlation_id/exchange`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct LinkOAuthCorrelationIdExchangeRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A `link_correlation_id` from a received OAuth redirect URI callback
    #[serde(rename = "link_correlation_id")]
    pub link_correlation_id: String,
}

impl LinkOAuthCorrelationIdExchangeRequest {
    /// LinkOAuthCorrelationIdExchangeRequest defines the request schema for `/link/oauth/correlation_id/exchange`
    pub fn new(link_correlation_id: String) -> LinkOAuthCorrelationIdExchangeRequest {
        LinkOAuthCorrelationIdExchangeRequest {
            client_id: None,
            secret: None,
            link_correlation_id,
        }
    }
}
