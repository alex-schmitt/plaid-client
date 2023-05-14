# TransactionsEnrichGetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**account_type** | **String** | The account type for the requested transactions (either `depository` or `credit`). | 
**transactions** | [**Vec<crate::models::ClientProvidedTransaction>**](ClientProvidedTransaction.md) | An array of transaction objects to be enriched by Plaid. Maximum of 100 transactions per request. | 
**options** | Option<[**crate::models::TransactionsEnrichRequestOptions**](TransactionsEnrichRequestOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


