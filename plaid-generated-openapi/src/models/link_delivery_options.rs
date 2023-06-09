/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryOptions : Optional metadata related to the link delivery session

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct LinkDeliveryOptions {
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Box<crate::models::LinkDeliveryRecipient>>,
}

impl LinkDeliveryOptions {
    /// Optional metadata related to the link delivery session
    pub fn new() -> LinkDeliveryOptions {
        LinkDeliveryOptions { recipient: None }
    }
}
