# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the account in which this transaction occurred. | 
**amount** | **f64** | The settled value of the transaction, denominated in the transactions's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 
**category** | Option<**Vec<String>**> | A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights. | 
**category_id** | Option<**String**> | The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights. | 
**check_number** | Option<**String**> | The check number of the transaction. This field is only populated for check transactions. | [optional]
**date** | [**String**](string.md) | For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ). | 
**location** | [**crate::models::Location**](Location.md) |  | 
**name** | **String** | The merchant name or transaction description.  If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, this field will always appear. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights. | 
**merchant_name** | Option<**String**> | The merchant name, as enriched by Plaid from the `name` field. This is typically a more human-readable version of the merchant counterparty in the transaction. For some bank transactions (such as checks or account transfers) where there is no meaningful merchant name, this value will be `null`. | [optional]
**original_description** | Option<**String**> | The string returned by the financial institution to describe the transaction. For transactions returned by `/transactions/get`, this field is in beta and will be omitted unless the client is both enrolled in the closed beta program and has set `options.include_original_description` to `true`. | [optional]
**payment_meta** | [**crate::models::PaymentMeta**](PaymentMeta.md) |  | 
**pending** | **bool** | When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled. | 
**pending_transaction_id** | Option<**String**> | The ID of a posted transaction's associated pending transaction, where applicable. | 
**account_owner** | Option<**String**> | The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts. | 
**transaction_id** | **String** | The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive. | 
**transaction_type** | Option<**String**> | Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.  `digital:` transactions that took place online.  `place:` transactions that were made at a physical location.  `special:` transactions that relate to banks, e.g. fees or deposits.  `unresolved:` transactions that do not fit into the other three types.  | [optional]
**logo_url** | Option<**String**> | The logo associated with the merchant, if available. Formatted as a 100x100 pixels PNG file path. | [optional]
**website** | Option<**String**> | The website associated with the merchant, if available. | [optional]
**authorized_date** | Option<[**String**](string.md)> | The date that the transaction was authorized. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ). | 
**authorized_datetime** | Option<**String**> | Date and time when a transaction was authorized in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). This field is only populated in API version 2019-05-29 and later. | 
**datetime** | Option<**String**> | Date and time when a transaction was posted in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). This field is only populated in API version 2019-05-29 and later. | 
**payment_channel** | **String** | The channel used to make a payment. `online:` transactions that took place online.  `in store:` transactions that were made at a physical location.  `other:` transactions that relate to banks, e.g. fees or deposits.  This field replaces the `transaction_type` field.  | 
**personal_finance_category** | Option<[**crate::models::PersonalFinanceCategory**](PersonalFinanceCategory.md)> |  | [optional]
**transaction_code** | Option<[**crate::models::TransactionCode**](TransactionCode.md)> |  | 
**personal_finance_category_icon_url** | Option<**String**> | A link to the icon associated with the primary personal finance category. The logo will always be 100x100 pixels. | [optional]
**counterparties** | Option<[**Vec<crate::models::TransactionCounterparty>**](TransactionCounterparty.md)> | The counterparties present in the transaction. Counterparties, such as the financial institutions, are extracted by Plaid from the raw description. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


