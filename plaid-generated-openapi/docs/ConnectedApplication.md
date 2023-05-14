# ConnectedApplication

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_id** | **String** | This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect. | 
**name** | **String** | The name of the application | 
**display_name** | Option<**String**> | A human-readable name of the application for display purposes | [optional]
**logo_url** | Option<**String**> | A URL that links to the application logo image. | [optional]
**application_url** | Option<**String**> | The URL for the application's website | [optional]
**reason_for_access** | Option<**String**> | A string provided by the connected app stating why they use their respective enabled products. | [optional]
**created_at** | [**String**](string.md) | The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC. | 
**scopes** | Option<[**crate::models::ScopesNullable**](ScopesNullable.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


