# PhysicalDocumentImages

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**original_front** | Option<**String**> | Temporary URL that expires after 60 seconds for downloading the uncropped original image of the front of the document. | 
**original_back** | Option<**String**> | Temporary URL that expires after 60 seconds for downloading the original image of the back of the document. Might be null if the back of the document was not collected. | 
**cropped_front** | Option<**String**> | Temporary URL that expires after 60 seconds for downloading a cropped image containing just the front of the document. | 
**cropped_back** | Option<**String**> | Temporary URL that expires after 60 seconds for downloading a cropped image containing just the back of the document. Might be null if the back of the document was not collected. | 
**face** | Option<**String**> | Temporary URL that expires after 60 seconds for downloading a crop of just the user's face from the document image. Might be null if the document does not contain a face photo. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


