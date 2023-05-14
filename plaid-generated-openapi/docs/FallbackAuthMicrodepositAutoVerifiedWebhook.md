# FallbackAuthMicrodepositAutoVerifiedWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `AUTH` | 
**webhook_code** | **String** | `AUTOMATICALLY_VERIFIED` | 
**error** | Option<**String**> | The error code associated with the webhook. | [optional]
**account_id** | **String** | The external account ID associated with the micro-deposit | 
**item_id** | **String** | The `item_id` of the Item associated with this webhook, warning, or error | 
**environment** | [**crate::models::WebhookEnvironmentValues**](WebhookEnvironmentValues.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


