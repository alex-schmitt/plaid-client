/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WalletStatus : The status of the wallet.  `UNKNOWN`: The wallet status is unknown.  `ACTIVE`: The wallet is active and ready to send money to and receive money from.  `CLOSED`: The wallet is closed. Any transactions made to or from this wallet will error.

/// The status of the wallet.  `UNKNOWN`: The wallet status is unknown.  `ACTIVE`: The wallet is active and ready to send money to and receive money from.  `CLOSED`: The wallet is closed. Any transactions made to or from this wallet will error.
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
pub enum WalletStatus {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "CLOSED")]
    Closed,
}

impl ToString for WalletStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("UNKNOWN"),
            Self::Active => String::from("ACTIVE"),
            Self::Closed => String::from("CLOSED"),
        }
    }
}

impl Default for WalletStatus {
    fn default() -> WalletStatus {
        Self::Unknown
    }
}
