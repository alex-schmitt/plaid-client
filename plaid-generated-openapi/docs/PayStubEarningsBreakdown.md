# PayStubEarningsBreakdown

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_description** | Option<**String**> | Commonly used term to describe the earning line item. | 
**current_amount** | Option<**f64**> | Raw amount of the earning line item. | 
**description** | Option<**String**> | Description of the earning line item. | 
**hours** | Option<**f32**> | Number of hours applicable for this earning. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null. | 
**rate** | Option<**f64**> | Hourly rate applicable for this earning. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 
**ytd_amount** | Option<**f64**> | The year-to-date amount of the deduction. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


