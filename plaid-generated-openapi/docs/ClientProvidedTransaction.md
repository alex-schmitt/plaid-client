# ClientProvidedTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique ID for the transaction used to help you tie data back to your systems. | 
**description** | **String** | The raw description of the transaction. If you have location data in available an unstructured format, it may be appended to the `description` field. | 
**amount** | **f64** | The absolute value of the transaction (>= 0). When testing Enrich, note that `amount` data should be realistic. Unrealistic or inaccurate `amount` data may result in reduced quality output. | 
**direction** | [**crate::models::EnrichTransactionDirection**](EnrichTransactionDirection.md) |  | 
**iso_currency_code** | **String** | The ISO-4217 currency code of the transaction e.g. USD. | 
**location** | Option<[**crate::models::ClientProvidedTransactionLocation**](ClientProvidedTransactionLocation.md)> |  | [optional]
**mcc** | Option<**String**> | Merchant category codes (MCCs) are four-digit numbers that describe a merchant's primary business activities. | [optional]
**date_posted** | Option<[**String**](string.md)> | The date the transaction posted, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


