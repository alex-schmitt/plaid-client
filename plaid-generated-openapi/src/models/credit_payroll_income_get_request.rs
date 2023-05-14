/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditPayrollIncomeGetRequest : CreditPayrollIncomeGetRequest defines the request schema for `/credit/payroll_income/get`.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditPayrollIncomeGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The user token associated with the User data is being requested for.
    #[serde(rename = "user_token", skip_serializing_if = "Option::is_none")]
    pub user_token: Option<String>,
}

impl CreditPayrollIncomeGetRequest {
    /// CreditPayrollIncomeGetRequest defines the request schema for `/credit/payroll_income/get`.
    pub fn new() -> CreditPayrollIncomeGetRequest {
        CreditPayrollIncomeGetRequest {
            client_id: None,
            secret: None,
            user_token: None,
        }
    }
}
