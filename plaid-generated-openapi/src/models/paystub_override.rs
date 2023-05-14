/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaystubOverride : An object representing data from a paystub.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PaystubOverride {
    #[serde(rename = "employer", skip_serializing_if = "Option::is_none")]
    pub employer: Option<Box<crate::models::PaystubOverrideEmployer>>,
    #[serde(rename = "employee", skip_serializing_if = "Option::is_none")]
    pub employee: Option<Box<crate::models::PaystubOverrideEmployee>>,
    #[serde(rename = "income_breakdown", skip_serializing_if = "Option::is_none")]
    pub income_breakdown: Option<Vec<crate::models::IncomeBreakdown>>,
    #[serde(rename = "pay_period_details", skip_serializing_if = "Option::is_none")]
    pub pay_period_details: Option<crate::models::PayPeriodDetails>,
}

impl PaystubOverride {
    /// An object representing data from a paystub.
    pub fn new() -> PaystubOverride {
        PaystubOverride {
            employer: None,
            employee: None,
            income_breakdown: None,
            pay_period_details: None,
        }
    }
}
