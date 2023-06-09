# AssetReportCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**access_tokens** | Option<**Vec<String>**> | An array of access tokens corresponding to the Items that will be included in the report. The `assets` product must have been initialized for the Items during link; the Assets product cannot be added after initialization. | [optional]
**user_token** | Option<**String**> | The user token associated with the User for which to create an asset report for. All items associated with the User will be included in the report. | [optional]
**days_requested** | **i32** | The maximum integer number of days of history to include in the Asset Report. If using Fannie Mae Day 1 Certainty, `days_requested` must be at least 61 for new originations or at least 31 for refinancings.  An Asset Report requested with \"Additional History\" (that is, with more than 61 days of transaction history) will incur an Additional History fee. | 
**options** | Option<[**crate::models::AssetReportCreateRequestOptions**](AssetReportCreateRequestOptions.md)> |  | [optional]
**report_type** | Option<[**crate::models::FreddieReportType**](FreddieReportType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


