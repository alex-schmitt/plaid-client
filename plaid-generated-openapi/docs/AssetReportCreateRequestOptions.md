# AssetReportCreateRequestOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_report_id** | Option<**String**> | Client-generated identifier, which can be used by lenders to track loan applications. | [optional]
**webhook** | Option<**String**> | URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready. | [optional]
**include_fast_report** | Option<**bool**> | true to return balance and identity earlier as a fast report. Defaults to false if omitted. | [optional]
**products** | Option<**Vec<String>**> | Additional information that can be included in the asset report. Possible values: `\"investments\"` | [optional]
**add_ons** | Option<[**Vec<crate::models::AssetReportAddOns>**](AssetReportAddOns.md)> | Additional information that can be included in the asset report. Possible values: `\"fast_assets\"` | [optional]
**user** | Option<[**crate::models::AssetReportUser**](AssetReportUser.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


