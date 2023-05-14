# ClientProvidedRawTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique ID for the transaction used to help you tie data back to your systems. | 
**description** | **String** | The raw description of the transaction. | 
**amount** | **f64** | The value of the transaction with direction. (NOTE: this will affect enrichment results, so directions are important):.   Negative (-) for credits (e.g., incoming transfers, refunds)   Positive (+) for debits (e.g., purchases, fees, outgoing transfers) | 
**iso_currency_code** | **String** | The ISO-4217 currency code of the transaction e.g. USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


