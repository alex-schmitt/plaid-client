# CreditBankIncomeSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_amount** | Option<**f32**> | Total amount of earnings across all the income sources in the end user's Items for the days requested by the client. This may return an incorrect value if the summary includes income sources in multiple currencies. Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-bank-income-summary-total-amounts) instead. | [optional]
**iso_currency_code** | Option<**String**> | The ISO 4217 currency code of the amount or balance. Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-bank-income-summary-total-amounts) instead. | [optional]
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries. Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-bank-income-summary-total-amounts) instead. | [optional]
**total_amounts** | Option<[**Vec<crate::models::CreditAmountWithCurrency>**](CreditAmountWithCurrency.md)> | Total amount of earnings across all the income sources in the end user's Items for the days requested by the client. This can contain multiple amounts, with each amount denominated in one unique currency. | [optional]
**start_date** | Option<[**String**](string.md)> | The earliest date within the days requested in which all income sources identified by Plaid appear in a user's account. The date will be returned in an ISO 8601 format (YYYY-MM-DD). | [optional]
**end_date** | Option<[**String**](string.md)> | The latest date in which all income sources identified by Plaid appear in the user's account. The date will be returned in an ISO 8601 format (YYYY-MM-DD). | [optional]
**income_sources_count** | Option<**i32**> | Number of income sources per end user. | [optional]
**income_categories_count** | Option<**i32**> | Number of income categories per end user. | [optional]
**income_transactions_count** | Option<**i32**> | Number of income transactions per end user. | [optional]
**historical_summary** | Option<[**Vec<crate::models::CreditBankIncomeHistoricalSummary>**](CreditBankIncomeHistoricalSummary.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


