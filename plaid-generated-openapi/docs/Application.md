# Application

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_id** | **String** | This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect. | 
**name** | **String** | The name of the application | 
**display_name** | Option<**String**> | A human-readable name of the application for display purposes | 
**join_date** | [**String**](string.md) | The date this application was granted production access at Plaid in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC. | 
**logo_url** | Option<**String**> | A URL that links to the application logo image. | 
**application_url** | Option<**String**> | The URL for the application's website | 
**reason_for_access** | Option<**String**> | A string provided by the connected app stating why they use their respective enabled products. | 
**use_case** | Option<**String**> | A string representing client’s broad use case as assessed by Plaid. | 
**company_legal_name** | Option<**String**> | A string representing the name of client’s legal entity. | 
**city** | Option<**String**> | A string representing the city of the client’s headquarters. | 
**region** | Option<**String**> | A string representing the region of the client’s headquarters. | 
**postal_code** | Option<**String**> | A string representing the postal code of the client’s headquarters. | 
**country_code** | Option<**String**> | A string representing the country code of the client’s headquarters. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


