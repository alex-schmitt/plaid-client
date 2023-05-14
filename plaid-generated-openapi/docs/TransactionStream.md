# TransactionStream

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the account to which the stream belongs | 
**stream_id** | **String** | A unique id for the stream | 
**category** | **Vec<String>** | A hierarchical array of the categories to which this transaction belongs. See [Categories](https://plaid.com/docs/api/products/transactions/#categoriesget). | 
**category_id** | **String** | The ID of the category to which this transaction belongs. See [Categories](https://plaid.com/docs/api/products/transactions/#categoriesget). | 
**description** | **String** | A description of the transaction stream. | 
**merchant_name** | Option<**String**> | The merchant associated with the transaction stream. | 
**first_date** | [**String**](string.md) | The posted date of the earliest transaction in the stream. | 
**last_date** | [**String**](string.md) | The posted date of the latest transaction in the stream. | 
**frequency** | [**crate::models::RecurringTransactionFrequency**](RecurringTransactionFrequency.md) |  | 
**transaction_ids** | **Vec<String>** | An array of Plaid transaction IDs belonging to the stream, sorted by posted date. | 
**average_amount** | [**crate::models::TransactionStreamAmount**](TransactionStreamAmount.md) |  | 
**last_amount** | [**crate::models::TransactionStreamAmount**](TransactionStreamAmount.md) |  | 
**is_active** | **bool** | Indicates whether the transaction stream is still live. | 
**status** | [**crate::models::TransactionStreamStatus**](TransactionStreamStatus.md) |  | 
**personal_finance_category** | Option<[**crate::models::PersonalFinanceCategory**](PersonalFinanceCategory.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


