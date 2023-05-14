# UpdateEntityScreeningRequestSearchTerms

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity_watchlist_program_id** | **String** | ID of the associated entity program. | 
**legal_name** | Option<**String**> | The name of the organization being screened. | [optional]
**document_number** | Option<**String**> | The numeric or alphanumeric identifier associated with this document. | [optional]
**email_address** | Option<**String**> | A valid email address. | [optional]
**country** | Option<**String**> | Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form. | [optional]
**phone_number** | Option<**String**> | A phone number in E.164 format. | [optional]
**url** | Option<**String**> | An 'http' or 'https' URL (must begin with either of those). | [optional]
**client_id** | **String** | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | 
**secret** | **String** | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


