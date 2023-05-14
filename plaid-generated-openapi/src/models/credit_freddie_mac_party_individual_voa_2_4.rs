/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacPartyIndividualVoa24 : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditFreddieMacPartyIndividualVoa24 {
    #[serde(rename = "NAME")]
    pub name: crate::models::CreditFreddieMacIndividualNameVoa24,
}

impl CreditFreddieMacPartyIndividualVoa24 {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(
        name: crate::models::CreditFreddieMacIndividualNameVoa24,
    ) -> CreditFreddieMacPartyIndividualVoa24 {
        CreditFreddieMacPartyIndividualVoa24 { name }
    }
}
