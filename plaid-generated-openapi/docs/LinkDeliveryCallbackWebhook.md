# LinkDeliveryCallbackWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `LINK_DELIVERY` | 
**webhook_code** | **String** | `LINK_CALLBACK` | 
**link_delivery_session_id** | **String** | The ID of the link delivery session. | 
**timestamp** | **String** | Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | 
**error** | Option<[**crate::models::PlaidError**](PlaidError.md)> |  | [optional]
**link_callback_metadata** | [**crate::models::LinkCallbackMetadata**](LinkCallbackMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


