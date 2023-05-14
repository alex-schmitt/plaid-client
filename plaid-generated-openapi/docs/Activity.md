# Activity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity** | [**crate::models::ActivityType**](ActivityType.md) |  | 
**initiated_date** | **String** | The date and time this activity was initiated [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC. | 
**id** | **String** | A unique identifier for the activity | 
**initiator** | **String** | Application ID of the client who initiated the activity. | 
**state** | [**crate::models::ActionState**](ActionState.md) |  | 
**target_application_id** | Option<**String**> | This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect. | [optional]
**scopes** | Option<[**crate::models::ScopesNullable**](ScopesNullable.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


