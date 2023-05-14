/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportCreateRequestOptions : An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct AssetReportCreateRequestOptions {
    /// Client-generated identifier, which can be used by lenders to track loan applications.
    #[serde(
        rename = "client_report_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_report_id: Option<Option<String>>,
    /// URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    #[serde(
        rename = "webhook",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook: Option<Option<String>>,
    /// true to return balance and identity earlier as a fast report. Defaults to false if omitted.
    #[serde(
        rename = "include_fast_report",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_fast_report: Option<Option<bool>>,
    /// Additional information that can be included in the asset report. Possible values: `\"investments\"`
    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    /// Additional information that can be included in the asset report. Possible values: `\"fast_assets\"`
    #[serde(rename = "add_ons", skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<crate::models::AssetReportAddOns>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::AssetReportUser>,
}

impl AssetReportCreateRequestOptions {
    /// An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.
    pub fn new() -> AssetReportCreateRequestOptions {
        AssetReportCreateRequestOptions {
            client_report_id: None,
            webhook: None,
            include_fast_report: None,
            products: None,
            add_ons: None,
            user: None,
        }
    }
}