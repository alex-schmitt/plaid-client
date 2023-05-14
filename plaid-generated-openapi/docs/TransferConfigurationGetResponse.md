# TransferConfigurationGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 
**max_single_transfer_amount** | **String** | The max limit of dollar amount of a single transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**max_daily_credit_amount** | **String** | The max limit of sum of dollar amount of credit transfers in last 24 hours (decimal string with two digits of precision e.g. \"10.00\"). | 
**max_daily_debit_amount** | **String** | The max limit of sum of dollar amount of debit transfers in last 24 hours (decimal string with two digits of precision e.g. \"10.00\"). | 
**max_monthly_amount** | **String** | The max limit of sum of dollar amount of credit and debit transfers in one calendar month (decimal string with two digits of precision e.g. \"10.00\"). | 
**iso_currency_code** | **String** | The currency of the dollar amount, e.g. \"USD\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


