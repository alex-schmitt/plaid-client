# TransactionsSyncResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**added** | [**Vec<crate::models::Transaction>**](Transaction.md) | Transactions that have been added to the Item since `cursor` ordered by ascending last modified time. | 
**modified** | [**Vec<crate::models::Transaction>**](Transaction.md) | Transactions that have been modified on the Item since `cursor` ordered by ascending last modified time. | 
**removed** | [**Vec<crate::models::RemovedTransaction>**](RemovedTransaction.md) | Transactions that have been removed from the Item since `cursor` ordered by ascending last modified time. | 
**next_cursor** | **String** | Cursor used for fetching any future updates after the latest update provided in this response. The cursor obtained after all pages have been pulled (indicated by `has_more` being `false`) will be valid for at least 1 year. This cursor should be persisted for later calls. If transactions are not yet available, this will be an empty string. | 
**has_more** | **bool** | Represents if more than requested count of transaction updates exist. If true, the additional updates can be fetched by making an additional request with `cursor` set to `next_cursor`. If `has_more` is true, it’s important to pull all available pages, to make it less likely for underlying data changes to conflict with pagination. | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


