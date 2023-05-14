# DocumentaryVerificationDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**crate::models::DocumentStatus**](DocumentStatus.md) |  | 
**attempt** | **f32** | The `attempt` field begins with 1 and increments with each subsequent document upload. | 
**images** | [**crate::models::PhysicalDocumentImages**](PhysicalDocumentImages.md) |  | 
**extracted_data** | Option<[**crate::models::PhysicalDocumentExtractedData**](PhysicalDocumentExtractedData.md)> |  | 
**analysis** | [**crate::models::DocumentAnalysis**](DocumentAnalysis.md) |  | 
**redacted_at** | Option<**String**> | An ISO8601 formatted timestamp. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


