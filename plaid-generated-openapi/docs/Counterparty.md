# Counterparty

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the counterparty, such as the merchant or the financial institution, as extracted by Plaid from the raw description. | 
**entity_id** | Option<**String**> | A unique, stable, Plaid-generated id that maps to the counterparty. | [optional]
**r#type** | [**crate::models::CounterpartyType**](CounterpartyType.md) |  | 
**website** | Option<**String**> | The website associated with the counterparty. | 
**logo_url** | Option<**String**> | The URL of a logo associated with the counterparty, if available. The logo is formatted as a 100x100 pixel PNG file. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


