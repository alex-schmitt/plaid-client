/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryCreateRequest : LinkDeliveryCreateRequest defines the request schema for `/link_delivery/create`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct LinkDeliveryCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A `link_token` from a previous invocation of `/link/token/create` with Link Delivery enabled.
    #[serde(rename = "link_token")]
    pub link_token: String,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::LinkDeliveryOptions>>,
}

impl LinkDeliveryCreateRequest {
    /// LinkDeliveryCreateRequest defines the request schema for `/link_delivery/create`
    pub fn new(link_token: String) -> LinkDeliveryCreateRequest {
        LinkDeliveryCreateRequest {
            client_id: None,
            secret: None,
            link_token,
            options: None,
        }
    }
}
