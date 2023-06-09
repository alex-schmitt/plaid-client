# SandboxPublicTokenCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**institution_id** | **String** | The ID of the institution the Item will be associated with | 
**initial_products** | [**Vec<crate::models::Products>**](Products.md) | The products to initially pull for the Item. May be any products that the specified `institution_id`  supports. This array may not be empty. | 
**options** | Option<[**crate::models::SandboxPublicTokenCreateRequestOptions**](SandboxPublicTokenCreateRequestOptions.md)> |  | [optional]
**user_token** | Option<**String**> | The user token associated with the User data is being requested for. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


