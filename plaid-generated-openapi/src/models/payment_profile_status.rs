/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaymentProfileStatus : The status of the given Payment Profile.  `READY`: This Payment Profile is ready to be used to create transfers using `/transfer/authorization/create` and `/transfer/create`.  `PENDING`: This Payment Profile is not ready to be used. You’ll need to call `/link/token/create` and provide the `payment_profile_token` in the `transfer.payment_profile_token` field to initiate the account linking experience.  `REMOVED`: This Payment Profile has been removed.

/// The status of the given Payment Profile.  `READY`: This Payment Profile is ready to be used to create transfers using `/transfer/authorization/create` and `/transfer/create`.  `PENDING`: This Payment Profile is not ready to be used. You’ll need to call `/link/token/create` and provide the `payment_profile_token` in the `transfer.payment_profile_token` field to initiate the account linking experience.  `REMOVED`: This Payment Profile has been removed.
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
pub enum PaymentProfileStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "REMOVED")]
    Removed,
}

impl ToString for PaymentProfileStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("PENDING"),
            Self::Ready => String::from("READY"),
            Self::Removed => String::from("REMOVED"),
        }
    }
}

impl Default for PaymentProfileStatus {
    fn default() -> PaymentProfileStatus {
        Self::Pending
    }
}