/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DetailedOriginator : Originator and their status.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct DetailedOriginator {
    /// Originator’s client ID.
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "transfer_diligence_status")]
    pub transfer_diligence_status: crate::models::TransferDiligenceStatus,
    #[serde(rename = "company_name")]
    pub company_name: String,
}

impl DetailedOriginator {
    /// Originator and their status.
    pub fn new(
        client_id: String,
        transfer_diligence_status: crate::models::TransferDiligenceStatus,
        company_name: String,
    ) -> DetailedOriginator {
        DetailedOriginator {
            client_id,
            transfer_diligence_status,
            company_name,
        }
    }
}
