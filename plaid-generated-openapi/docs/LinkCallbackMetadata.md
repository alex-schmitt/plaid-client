# LinkCallbackMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_type** | Option<[**crate::models::LinkDeliveryWebhookCallbackType**](LinkDeliveryWebhookCallbackType.md)> |  | [optional]
**event_name** | Option<[**crate::models::LinkEventName**](LinkEventName.md)> |  | [optional]
**status** | Option<**String**> | Indicates where in the flow the Link user exited | [optional]
**link_session_id** | Option<**String**> | A unique identifier associated with a user's actions and events through the Link flow. Include this identifier when opening a support ticket for faster turnaround. | [optional]
**request_id** | Option<**String**> | The request ID for the last request made by Link. This can be shared with Plaid Support to expedite investigation. | [optional]
**institution** | Option<[**crate::models::LinkDeliveryInstitution**](LinkDeliveryInstitution.md)> |  | [optional]
**accounts** | Option<[**Vec<crate::models::LinkDeliveryAccount>**](LinkDeliveryAccount.md)> | A list of accounts attached to the connected Item. If Account Select is enabled via the developer dashboard, accounts will only include selected accounts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


