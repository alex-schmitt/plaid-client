# PaymentInitiationRecipientGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipient_id** | **String** | The ID of the recipient. | 
**name** | **String** | The name of the recipient. | 
**address** | Option<[**crate::models::PaymentInitiationAddress**](PaymentInitiationAddress.md)> |  | [optional]
**iban** | Option<**String**> | The International Bank Account Number (IBAN) for the recipient. | [optional]
**bacs** | Option<[**crate::models::RecipientBacsNullable**](RecipientBACSNullable.md)> |  | [optional]
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


