# IdentityVerificationRequestUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_user_id** | **String** | An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object. | 
**email_address** | Option<**String**> | A valid email address. | [optional]
**phone_number** | Option<**String**> | A phone number in E.164 format. | [optional]
**date_of_birth** | Option<[**String**](string.md)> | A date in the format YYYY-MM-DD (RFC 3339 Section 5.6). | [optional]
**name** | Option<[**crate::models::IdentityVerificationRequestUserName**](IdentityVerificationRequestUserName.md)> |  | [optional]
**address** | Option<[**crate::models::UserAddress**](UserAddress.md)> |  | [optional]
**id_number** | Option<[**crate::models::UserIdNumber**](UserIDNumber.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


