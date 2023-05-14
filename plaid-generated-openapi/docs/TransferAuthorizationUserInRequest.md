# TransferAuthorizationUserInRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**legal_name** | **String** | The user's legal name. | 
**phone_number** | Option<**String**> | The user's phone number. In order to qualify for a guaranteed transfer, at least one of `phone_number` or `email_address` must be provided. | [optional]
**email_address** | Option<**String**> | The user's email address. In order to qualify for a guaranteed transfer, at least one of `phone_number` or `email_address` must be provided. | [optional]
**address** | Option<[**crate::models::TransferUserAddressInRequest**](TransferUserAddressInRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


