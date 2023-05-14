/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferCapabilitiesGetRtp : Contains the supported service types in RTP

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferCapabilitiesGetRtp {
    /// When `true`, the linked Item's institution supports RTP credit transfer.
    #[serde(rename = "credit", skip_serializing_if = "Option::is_none")]
    pub credit: Option<bool>,
}

impl TransferCapabilitiesGetRtp {
    /// Contains the supported service types in RTP
    pub fn new() -> TransferCapabilitiesGetRtp {
        TransferCapabilitiesGetRtp { credit: None }
    }
}
