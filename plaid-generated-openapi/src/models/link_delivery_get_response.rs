/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryGetResponse : LinkDeliveryGetRequest defines the response schema for `/link_delivery/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct LinkDeliveryGetResponse {
    #[serde(rename = "status")]
    pub status: crate::models::LinkDeliverySessionStatus,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Link Delivery Session was created at.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Link Delivery Session was completed at.
    #[serde(
        rename = "completed_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at: Option<Option<String>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    /// An array of access tokens associated with the Link Delivery session.
    #[serde(
        rename = "access_tokens",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_tokens: Option<Option<Vec<String>>>,
    /// An array of `item_id`s associated with the Link Delivery session.
    #[serde(
        rename = "item_ids",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub item_ids: Option<Option<Vec<String>>>,
}

impl LinkDeliveryGetResponse {
    /// LinkDeliveryGetRequest defines the response schema for `/link_delivery/get`
    pub fn new(
        status: crate::models::LinkDeliverySessionStatus,
        created_at: String,
        request_id: String,
    ) -> LinkDeliveryGetResponse {
        LinkDeliveryGetResponse {
            status,
            created_at,
            completed_at: None,
            request_id,
            access_tokens: None,
            item_ids: None,
        }
    }
}