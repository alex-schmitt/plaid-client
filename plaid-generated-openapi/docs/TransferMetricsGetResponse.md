# TransferMetricsGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 
**daily_debit_transfer_volume** | **String** | Sum of dollar amount of debit transfers in last 24 hours (decimal string with two digits of precision e.g. \"10.00\"). | 
**daily_credit_transfer_volume** | **String** | Sum of dollar amount of credit transfers in last 24 hours (decimal string with two digits of precision e.g. \"10.00\"). | 
**monthly_transfer_volume** | **String** | Sum of dollar amount of credit and debit transfers in current calendar month (decimal string with two digits of precision e.g. \"10.00\"). | 
**iso_currency_code** | **String** | The currency of the dollar amount, e.g. \"USD\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


