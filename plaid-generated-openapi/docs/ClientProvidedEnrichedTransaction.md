# ClientProvidedEnrichedTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique ID for the transaction as provided by you in the request. | 
**description** | **String** | The raw description of the transaction. | 
**amount** | **f64** | The absolute value of the transaction (>= 0) | 
**direction** | Option<[**crate::models::EnrichTransactionDirection**](EnrichTransactionDirection.md)> |  | [optional]
**iso_currency_code** | **String** | The ISO-4217 currency code of the transaction e.g. USD. | 
**enrichments** | [**crate::models::Enrichments**](Enrichments.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


