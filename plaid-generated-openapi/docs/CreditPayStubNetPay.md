# CreditPayStubNetPay

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_amount** | Option<**f64**> | Raw amount of the net pay for the pay period. | 
**description** | Option<**String**> | Description of the net pay. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 
**ytd_amount** | Option<**f64**> | The year-to-date amount of the net pay. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


