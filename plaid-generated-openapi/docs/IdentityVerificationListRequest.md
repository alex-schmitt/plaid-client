# IdentityVerificationListRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**template_id** | **String** | ID of the associated Identity Verification template. | 
**client_user_id** | **String** | An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object. | 
**cursor** | Option<**String**> | An identifier that determines which page of results you receive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


