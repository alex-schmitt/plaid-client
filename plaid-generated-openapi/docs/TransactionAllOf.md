# TransactionAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authorized_date** | Option<[**String**](string.md)> | The date that the transaction was authorized. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ). | 
**authorized_datetime** | Option<**String**> | Date and time when a transaction was authorized in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). This field is only populated in API version 2019-05-29 and later. | 
**datetime** | Option<**String**> | Date and time when a transaction was posted in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). This field is only populated in API version 2019-05-29 and later. | 
**payment_channel** | **String** | The channel used to make a payment. `online:` transactions that took place online.  `in store:` transactions that were made at a physical location.  `other:` transactions that relate to banks, e.g. fees or deposits.  This field replaces the `transaction_type` field.  | 
**personal_finance_category** | Option<[**crate::models::PersonalFinanceCategory**](PersonalFinanceCategory.md)> |  | [optional]
**transaction_code** | Option<[**crate::models::TransactionCode**](TransactionCode.md)> |  | 
**personal_finance_category_icon_url** | Option<**String**> | A link to the icon associated with the primary personal finance category. The logo will always be 100x100 pixels. | [optional]
**counterparties** | Option<[**Vec<crate::models::TransactionCounterparty>**](TransactionCounterparty.md)> | The counterparties present in the transaction. Counterparties, such as the financial institutions, are extracted by Plaid from the raw description. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


