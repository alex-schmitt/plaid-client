# DocumentRiskSignal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The result from the risk signal check. | 
**field** | Option<**String**> | The field which the risk signal was computed for | 
**has_fraud_risk** | Option<**bool**> | A flag used to quickly identify if the signal indicates that this field is authentic or fraudulent | 
**institution_metadata** | Option<[**crate::models::DocumentRiskSignalInstitutionMetadata**](DocumentRiskSignalInstitutionMetadata.md)> |  | 
**expected_value** | Option<**String**> | The expected value of the field, as seen on the document | 
**actual_value** | Option<**String**> | The derived value obtained in the risk signal calculation process for this field | 
**signal_description** | Option<**String**> | A human-readable explanation providing more detail into the particular risk signal | 
**page_number** | Option<**i32**> | The relevant page associated with the risk signal | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


