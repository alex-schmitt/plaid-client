# WalletTransactionListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transactions** | [**Vec<crate::models::WalletTransaction>**](WalletTransaction.md) | An array of transactions of an e-wallet, associated with the given `wallet_id` | 
**next_cursor** | Option<**String**> | Cursor used for fetching transactions created before the latest transaction provided in this response | [optional]
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


