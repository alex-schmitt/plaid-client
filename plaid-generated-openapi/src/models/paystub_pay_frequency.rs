/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaystubPayFrequency : The frequency at which the employee is paid. Possible values: `MONTHLY`, `BI-WEEKLY`, `WEEKLY`, `SEMI-MONTHLY`.

/// The frequency at which the employee is paid. Possible values: `MONTHLY`, `BI-WEEKLY`, `WEEKLY`, `SEMI-MONTHLY`.
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
pub enum PaystubPayFrequency {
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "BI-WEEKLY")]
    BiWeekly,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "SEMI-MONTHLY")]
    SemiMonthly,
    #[serde(rename = "null")]
    Null,
}

impl ToString for PaystubPayFrequency {
    fn to_string(&self) -> String {
        match self {
            Self::Monthly => String::from("MONTHLY"),
            Self::BiWeekly => String::from("BI-WEEKLY"),
            Self::Weekly => String::from("WEEKLY"),
            Self::SemiMonthly => String::from("SEMI-MONTHLY"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for PaystubPayFrequency {
    fn default() -> PaystubPayFrequency {
        Self::Monthly
    }
}
