# WalletTransactionStatusUpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `WALLET` | 
**webhook_code** | **String** | `WALLET_TRANSACTION_STATUS_UPDATE` | 
**transaction_id** | **String** | The `transaction_id` for the wallet transaction being updated | 
**payment_id** | Option<**String**> | The `payment_id` associated with the transaction. This will be present in case of `REFUND` and `PIS_PAY_IN` | [optional]
**wallet_id** | Option<**String**> | The EMI (E-Money Institution) wallet that this payment is associated with. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests. | [optional]
**new_status** | [**crate::models::WalletTransactionStatus**](WalletTransactionStatus.md) |  | 
**old_status** | [**crate::models::WalletTransactionStatus**](WalletTransactionStatus.md) |  | 
**timestamp** | **String** | The timestamp of the update, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `\"2017-09-14T14:42:19.350Z\"` | 
**environment** | [**crate::models::WebhookEnvironmentValues**](WebhookEnvironmentValues.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


