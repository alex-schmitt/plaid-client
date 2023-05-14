# TransferAuthorizationProposedTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ach_class** | Option<[**crate::models::AchClass**](ACHClass.md)> |  | [optional]
**account_id** | Option<**String**> | The Plaid `account_id` for the account that will be debited or credited. | [optional]
**funding_account_id** | **String** | The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited. | 
**r#type** | [**crate::models::TransferType**](TransferType.md) |  | 
**user** | [**crate::models::TransferUserInResponse**](TransferUserInResponse.md) |  | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**network** | **String** | The network or rails used for the transfer. | 
**origination_account_id** | **String** | Plaid's unique identifier for the origination account that was used for this transfer. | 
**iso_currency_code** | **String** | The currency of the transfer amount. The default value is \"USD\". | 
**originator_client_id** | Option<**String**> | The Plaid client ID that is the originator of this transfer. Only present if created on behalf of another client as a third-party sender (TPS). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


