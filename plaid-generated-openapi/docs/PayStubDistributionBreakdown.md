# PayStubDistributionBreakdown

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_name** | Option<**String**> | Name of the account for the given distribution. | 
**bank_name** | Option<**String**> | The name of the bank that the payment is being deposited to. | 
**current_amount** | Option<**f64**> | The amount distributed to this account. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null. | 
**mask** | Option<**String**> | The last 2-4 alphanumeric characters of an account's official account number. | 
**r#type** | Option<**String**> | Type of the account that the paystub was sent to (e.g. 'checking'). | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


