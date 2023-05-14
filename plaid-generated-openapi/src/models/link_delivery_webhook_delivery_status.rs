/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryWebhookDeliveryStatus : The status of the delivery of the hosted link to the user

/// The status of the delivery of the hosted link to the user
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
pub enum LinkDeliveryWebhookDeliveryStatus {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

impl ToString for LinkDeliveryWebhookDeliveryStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Succeeded => String::from("succeeded"),
            Self::Failed => String::from("failed"),
        }
    }
}

impl Default for LinkDeliveryWebhookDeliveryStatus {
    fn default() -> LinkDeliveryWebhookDeliveryStatus {
        Self::Succeeded
    }
}
