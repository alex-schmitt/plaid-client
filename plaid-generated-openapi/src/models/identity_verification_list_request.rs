/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityVerificationListRequest : Request input for listing identity verifications

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct IdentityVerificationListRequest {
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ID of the associated Identity Verification template.
    #[serde(rename = "template_id")]
    pub template_id: String,
    /// An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    #[serde(rename = "client_user_id")]
    pub client_user_id: String,
    /// An identifier that determines which page of results you receive.
    #[serde(
        rename = "cursor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cursor: Option<Option<String>>,
}

impl IdentityVerificationListRequest {
    /// Request input for listing identity verifications
    pub fn new(template_id: String, client_user_id: String) -> IdentityVerificationListRequest {
        IdentityVerificationListRequest {
            secret: None,
            client_id: None,
            template_id,
            client_user_id,
            cursor: None,
        }
    }
}
