/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportGetResponse : AssetReportGetResponse defines the response schema for `/asset_report/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct AssetReportGetResponse {
    #[serde(rename = "report")]
    pub report: crate::models::AssetReport,
    /// If the Asset Report generation was successful but identity information cannot be returned, this array will contain information about the errors causing identity information to be missing
    #[serde(rename = "warnings")]
    pub warnings: Vec<crate::models::Warning>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl AssetReportGetResponse {
    /// AssetReportGetResponse defines the response schema for `/asset_report/get`
    pub fn new(
        report: crate::models::AssetReport,
        warnings: Vec<crate::models::Warning>,
        request_id: String,
    ) -> AssetReportGetResponse {
        AssetReportGetResponse {
            report,
            warnings,
            request_id,
        }
    }
}