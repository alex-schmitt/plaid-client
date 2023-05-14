/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WalletGetResponse : WalletGetResponse defines the response schema for `/wallet/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct WalletGetResponse {
    /// A unique ID identifying the e-wallet
    #[serde(rename = "wallet_id")]
    pub wallet_id: String,
    #[serde(rename = "balance")]
    pub balance: crate::models::WalletBalance,
    #[serde(rename = "numbers")]
    pub numbers: crate::models::WalletNumbers,
    /// The ID of the recipient that corresponds to the e-wallet account numbers
    #[serde(rename = "recipient_id", skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    #[serde(rename = "status")]
    pub status: crate::models::WalletStatus,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl WalletGetResponse {
    /// WalletGetResponse defines the response schema for `/wallet/get`
    pub fn new(
        wallet_id: String,
        balance: crate::models::WalletBalance,
        numbers: crate::models::WalletNumbers,
        status: crate::models::WalletStatus,
        request_id: String,
    ) -> WalletGetResponse {
        WalletGetResponse {
            wallet_id,
            balance,
            numbers,
            recipient_id: None,
            status,
            request_id,
        }
    }
}