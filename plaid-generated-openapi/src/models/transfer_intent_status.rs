/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentStatus : The status of the transfer intent.  `PENDING`: The transfer intent is pending. `SUCCEEDED`: The transfer intent was successfully created. `FAILED`: The transfer intent was unable to be created.

/// The status of the transfer intent.  `PENDING`: The transfer intent is pending. `SUCCEEDED`: The transfer intent was successfully created. `FAILED`: The transfer intent was unable to be created.
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
pub enum TransferIntentStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "FAILED")]
    Failed,
}

impl ToString for TransferIntentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("PENDING"),
            Self::Succeeded => String::from("SUCCEEDED"),
            Self::Failed => String::from("FAILED"),
        }
    }
}

impl Default for TransferIntentStatus {
    fn default() -> TransferIntentStatus {
        Self::Pending
    }
}
