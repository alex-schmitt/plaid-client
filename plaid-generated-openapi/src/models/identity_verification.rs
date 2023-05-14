/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityVerification : A identity verification attempt represents a customer's attempt to verify their identity, reflecting the required steps for completing the session, the results for each step, and information collected in the process.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct IdentityVerification {
    /// ID of the associated Identity Verification attempt.
    #[serde(rename = "id")]
    pub id: String,
    /// An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    #[serde(rename = "client_user_id")]
    pub client_user_id: String,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
    /// The ID for the Identity Verification preceding this session. This field will only be filled if the current Identity Verification is a retry of a previous attempt.
    #[serde(
        rename = "previous_attempt_id",
        deserialize_with = "Option::deserialize"
    )]
    pub previous_attempt_id: Option<String>,
    /// A shareable URL that can be sent directly to the user to complete verification
    #[serde(rename = "shareable_url", deserialize_with = "Option::deserialize")]
    pub shareable_url: Option<String>,
    #[serde(rename = "template")]
    pub template: crate::models::IdentityVerificationTemplateReference,
    #[serde(rename = "user")]
    pub user: crate::models::IdentityVerificationUserData,
    #[serde(rename = "status")]
    pub status: crate::models::IdentityVerificationStatus,
    #[serde(rename = "steps")]
    pub steps: crate::models::IdentityVerificationStepSummary,
    #[serde(
        rename = "documentary_verification",
        deserialize_with = "Option::deserialize"
    )]
    pub documentary_verification: Option<crate::models::DocumentaryVerification>,
    #[serde(rename = "kyc_check", deserialize_with = "Option::deserialize")]
    pub kyc_check: Option<crate::models::KycCheckDetails>,
    #[serde(rename = "risk_check", deserialize_with = "Option::deserialize")]
    pub risk_check: Option<crate::models::RiskCheckDetails>,
    /// ID of the associated screening.
    #[serde(
        rename = "watchlist_screening_id",
        deserialize_with = "Option::deserialize"
    )]
    pub watchlist_screening_id: Option<String>,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "redacted_at", deserialize_with = "Option::deserialize")]
    pub redacted_at: Option<String>,
}

impl IdentityVerification {
    /// A identity verification attempt represents a customer's attempt to verify their identity, reflecting the required steps for completing the session, the results for each step, and information collected in the process.
    pub fn new(
        id: String,
        client_user_id: String,
        created_at: String,
        completed_at: Option<String>,
        previous_attempt_id: Option<String>,
        shareable_url: Option<String>,
        template: crate::models::IdentityVerificationTemplateReference,
        user: crate::models::IdentityVerificationUserData,
        status: crate::models::IdentityVerificationStatus,
        steps: crate::models::IdentityVerificationStepSummary,
        documentary_verification: Option<crate::models::DocumentaryVerification>,
        kyc_check: Option<crate::models::KycCheckDetails>,
        risk_check: Option<crate::models::RiskCheckDetails>,
        watchlist_screening_id: Option<String>,
        redacted_at: Option<String>,
    ) -> IdentityVerification {
        IdentityVerification {
            id,
            client_user_id,
            created_at,
            completed_at,
            previous_attempt_id,
            shareable_url,
            template,
            user,
            status,
            steps,
            documentary_verification,
            kyc_check,
            risk_check,
            watchlist_screening_id,
            redacted_at,
        }
    }
}
