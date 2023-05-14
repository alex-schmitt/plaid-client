# UserAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**street** | **String** | The primary street portion of an address. If the user has submitted their address, this field will always be filled. | 
**street2** | Option<**String**> | Extra street information, like an apartment or suite number. | [optional]
**city** | **String** | City from the end user's address | 
**region** | **String** | An ISO 3166-2 subdivision code. Related terms would be \"state\", \"province\", \"prefecture\", \"zone\", \"subdivision\", etc. | 
**postal_code** | **String** | The postal code for the associated address. Between 2 and 10 alphanumeric characters. For US-based addresses this must be 5 numeric digits. | 
**country** | **String** | Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


