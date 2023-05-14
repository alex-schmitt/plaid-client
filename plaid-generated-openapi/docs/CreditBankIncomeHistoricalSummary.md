# CreditBankIncomeHistoricalSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_amount** | Option<**f32**> | Total amount of earnings for the income source(s) of the user for the month in the summary. This may return an incorrect value if the summary includes income sources in multiple currencies. Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-items-bank-income-sources-historical-summary-total-amounts) instead. | [optional]
**iso_currency_code** | Option<**String**> | The ISO 4217 currency code of the amount or balance. Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-items-bank-income-sources-historical-summary-total-amounts) instead. | [optional]
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries. Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-items-bank-income-sources-historical-summary-total-amounts) instead. | [optional]
**total_amounts** | Option<[**Vec<crate::models::CreditAmountWithCurrency>**](CreditAmountWithCurrency.md)> | Total amount of earnings for the income source(s) of the user for the month in the summary. This can contain multiple amounts, with each amount denominated in one unique currency. | [optional]
**start_date** | Option<[**String**](string.md)> | The start date of the period covered in this monthly summary. This date will be the first day of the month, unless the month being covered is a partial month because it is the first month included in the summary and the date range being requested does not begin with the first day of the month. The date will be returned in an ISO 8601 format (YYYY-MM-DD). | [optional]
**end_date** | Option<[**String**](string.md)> | The end date of the period included in this monthly summary. This date will be the last day of the month, unless the month being covered is a partial month because it is the last month included in the summary and the date range being requested does not end with the last day of the month. The date will be returned in an ISO 8601 format (YYYY-MM-DD). | [optional]
**transactions** | Option<[**Vec<crate::models::CreditBankIncomeTransaction>**](CreditBankIncomeTransaction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


