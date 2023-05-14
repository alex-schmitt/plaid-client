# SignalEvaluateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 
**scores** | [**crate::models::SignalScores**](SignalScores.md) |  | 
**core_attributes** | Option<[**crate::models::SignalEvaluateCoreAttributes**](SignalEvaluateCoreAttributes.md)> |  | [optional]
**warnings** | Option<[**Vec<crate::models::SignalWarning>**](SignalWarning.md)> | If bank information was not able to be used as features into the Signal model, this array contains warnings describing why we were missing bank data | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


