# WalletCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wallet_id** | **String** | A unique ID identifying the e-wallet | 
**balance** | [**crate::models::WalletBalance**](WalletBalance.md) |  | 
**numbers** | [**crate::models::WalletNumbers**](WalletNumbers.md) |  | 
**recipient_id** | Option<**String**> | The ID of the recipient that corresponds to the e-wallet account numbers | [optional]
**status** | [**crate::models::WalletStatus**](WalletStatus.md) |  | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


