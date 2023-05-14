/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DepositoryFilter : A filter to apply to `depository`-type accounts

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct DepositoryFilter {
    /// An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    #[serde(rename = "account_subtypes")]
    pub account_subtypes: Vec<crate::models::DepositoryAccountSubtype>,
}

impl DepositoryFilter {
    /// A filter to apply to `depository`-type accounts
    pub fn new(account_subtypes: Vec<crate::models::DepositoryAccountSubtype>) -> DepositoryFilter {
        DepositoryFilter { account_subtypes }
    }
}
