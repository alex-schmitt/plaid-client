# TransferEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **i32** | Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers. | 
**timestamp** | **String** | The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`. | 
**event_type** | [**crate::models::TransferEventType**](TransferEventType.md) |  | 
**account_id** | **String** | The account ID associated with the transfer. | 
**funding_account_id** | **String** | The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited. | 
**transfer_id** | **String** | Plaid’s unique identifier for a transfer. | 
**origination_account_id** | Option<**String**> | The ID of the origination account that this balance belongs to. | 
**transfer_type** | [**crate::models::TransferType**](TransferType.md) |  | 
**transfer_amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**failure_reason** | Option<[**crate::models::TransferFailure**](TransferFailure.md)> |  | 
**sweep_id** | Option<**String**> | Plaid’s unique identifier for a sweep. | 
**sweep_amount** | Option<**String**> | A signed amount of how much was `swept` or `return_swept` (decimal string with two digits of precision e.g. \"-5.50\"). | 
**refund_id** | Option<**String**> | Plaid’s unique identifier for a refund. A non-null value indicates the event is for the associated refund of the transfer. | 
**originator_client_id** | Option<**String**> | The Plaid client ID that is the originator of the transfer that this event applies to. Only present if the transfer was created on behalf of another client as a third-party sender (TPS). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


