/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaystubYtdDetails : The amount of income earned year to date, as based on paystub data.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PaystubYtdDetails {
    /// Year-to-date gross earnings.
    #[serde(
        rename = "gross_earnings",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_earnings: Option<Option<f64>>,
    /// Year-to-date net (take home) earnings.
    #[serde(
        rename = "net_earnings",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_earnings: Option<Option<f64>>,
}

impl PaystubYtdDetails {
    /// The amount of income earned year to date, as based on paystub data.
    pub fn new() -> PaystubYtdDetails {
        PaystubYtdDetails {
            gross_earnings: None,
            net_earnings: None,
        }
    }
}
