# PayStubPayPeriodDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pay_amount** | Option<**f64**> | The amount of the paycheck. | 
**distribution_breakdown** | [**Vec<crate::models::PayStubDistributionBreakdown>**](PayStubDistributionBreakdown.md) |  | 
**end_date** | Option<[**String**](string.md)> | The date on which the pay period ended, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\"). | 
**gross_earnings** | Option<**f64**> | Total earnings before tax/deductions. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null. | 
**pay_date** | Option<[**String**](string.md)> | The date on which the pay stub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\"). | 
**pay_frequency** | Option<**String**> | The frequency at which an individual is paid. | 
**pay_basis** | Option<[**crate::models::CreditPayStubPayBasisType**](CreditPayStubPayBasisType.md)> |  | [optional]
**start_date** | Option<[**String**](string.md)> | The date on which the pay period started, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\"). | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


