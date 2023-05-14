/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportAuditCopyCreateResponse : AssetReportAuditCopyCreateResponse defines the response schema for `/asset_report/audit_copy/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct AssetReportAuditCopyCreateResponse {
    /// A token that can be shared with a third party auditor to allow them to obtain access to the Asset Report. This token should be stored securely.
    #[serde(rename = "audit_copy_token")]
    pub audit_copy_token: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl AssetReportAuditCopyCreateResponse {
    /// AssetReportAuditCopyCreateResponse defines the response schema for `/asset_report/audit_copy/get`
    pub fn new(audit_copy_token: String, request_id: String) -> AssetReportAuditCopyCreateResponse {
        AssetReportAuditCopyCreateResponse {
            audit_copy_token,
            request_id,
        }
    }
}
