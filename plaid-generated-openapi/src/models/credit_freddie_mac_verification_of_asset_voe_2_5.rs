/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacVerificationOfAssetVoe25 : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct CreditFreddieMacVerificationOfAssetVoe25 {
    #[serde(rename = "REPORTING_INFORMATION")]
    pub reporting_information: crate::models::CreditFreddieMacReportingInformationVoa24,
    #[serde(rename = "SERVICE_PRODUCT_FULFILLMENT")]
    pub service_product_fulfillment: crate::models::ServiceProductFulfillment,
    #[serde(rename = "VERIFICATION_OF_ASSET_RESPONSE")]
    pub verification_of_asset_response:
        crate::models::CreditFreddieMacVerificationOfAssetResponseVoe25,
}

impl CreditFreddieMacVerificationOfAssetVoe25 {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(
        reporting_information: crate::models::CreditFreddieMacReportingInformationVoa24,
        service_product_fulfillment: crate::models::ServiceProductFulfillment,
        verification_of_asset_response: crate::models::CreditFreddieMacVerificationOfAssetResponseVoe25,
    ) -> CreditFreddieMacVerificationOfAssetVoe25 {
        CreditFreddieMacVerificationOfAssetVoe25 {
            reporting_information,
            service_product_fulfillment,
            verification_of_asset_response,
        }
    }
}