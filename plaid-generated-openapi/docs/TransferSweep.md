# TransferSweep

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the sweep. | 
**funding_account_id** | **String** | The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited. | 
**created** | **String** | The datetime when the sweep occurred, in RFC 3339 format. | 
**amount** | **String** | Signed decimal amount of the sweep as it appears on your sweep account ledger (e.g. \"-10.00\")  If amount is not present, the sweep was net-settled to zero and outstanding debits and credits between the sweep account and Plaid are balanced. | 
**iso_currency_code** | **String** | The currency of the sweep, e.g. \"USD\". | 
**settled** | Option<[**String**](string.md)> | The date when the sweep settled, in the YYYY-MM-DD format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


