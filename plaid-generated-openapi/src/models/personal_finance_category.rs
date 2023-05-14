/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PersonalFinanceCategory : Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.  See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PersonalFinanceCategory {
    /// A high level category that communicates the broad category of the transaction.
    #[serde(rename = "primary")]
    pub primary: String,
    /// A granular category conveying the transaction's intent. This field can also be used as a unique identifier for the category.
    #[serde(rename = "detailed")]
    pub detailed: String,
}

impl PersonalFinanceCategory {
    /// Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.  See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.
    pub fn new(primary: String, detailed: String) -> PersonalFinanceCategory {
        PersonalFinanceCategory { primary, detailed }
    }
}