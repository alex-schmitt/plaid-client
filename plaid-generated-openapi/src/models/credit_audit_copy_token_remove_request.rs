/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditAuditCopyTokenRemoveRequest : CreditAuditCopyTokenRemoveRequest defines the request schema for `/credit/audit_copy_token/remove`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditAuditCopyTokenRemoveRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The `audit_copy_token` granting access to the Audit Copy you would like to revoke.
    #[serde(rename = "audit_copy_token")]
    pub audit_copy_token: String,
}

impl CreditAuditCopyTokenRemoveRequest {
    /// CreditAuditCopyTokenRemoveRequest defines the request schema for `/credit/audit_copy_token/remove`
    pub fn new(audit_copy_token: String) -> CreditAuditCopyTokenRemoveRequest {
        CreditAuditCopyTokenRemoveRequest {
            client_id: None,
            secret: None,
            audit_copy_token,
        }
    }
}
