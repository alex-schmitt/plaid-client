/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LastDataAccessTimes : Describes the last time each datatype was accessed by an application.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct LastDataAccessTimes {
    /// ID of the application accessing data.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// The last time account_balance_info was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(
        rename = "account_balance_info",
        deserialize_with = "Option::deserialize"
    )]
    pub account_balance_info: Option<String>,
    /// The last time account_routing_number was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(
        rename = "account_routing_number",
        deserialize_with = "Option::deserialize"
    )]
    pub account_routing_number: Option<String>,
    /// The last time contact_details was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(rename = "contact_details", deserialize_with = "Option::deserialize")]
    pub contact_details: Option<String>,
    /// The last time transactions was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(rename = "transactions", deserialize_with = "Option::deserialize")]
    pub transactions: Option<String>,
    /// The last time credit_and_loans was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(rename = "credit_and_loans", deserialize_with = "Option::deserialize")]
    pub credit_and_loans: Option<String>,
    /// The last time investments was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(rename = "investments", deserialize_with = "Option::deserialize")]
    pub investments: Option<String>,
    /// The last time payroll_info was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(rename = "payroll_info", deserialize_with = "Option::deserialize")]
    pub payroll_info: Option<String>,
    /// The last time transaction_risk_info was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(
        rename = "transaction_risk_info",
        deserialize_with = "Option::deserialize"
    )]
    pub transaction_risk_info: Option<String>,
}

impl LastDataAccessTimes {
    /// Describes the last time each datatype was accessed by an application.
    pub fn new(
        application_id: String,
        account_balance_info: Option<String>,
        account_routing_number: Option<String>,
        contact_details: Option<String>,
        transactions: Option<String>,
        credit_and_loans: Option<String>,
        investments: Option<String>,
        payroll_info: Option<String>,
        transaction_risk_info: Option<String>,
    ) -> LastDataAccessTimes {
        LastDataAccessTimes {
            application_id,
            account_balance_info,
            account_routing_number,
            contact_details,
            transactions,
            credit_and_loans,
            investments,
            payroll_info,
            transaction_risk_info,
        }
    }
}
