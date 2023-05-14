# DocumentMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the document. | [optional]
**status** | Option<**String**> | The processing status of the document.  `PROCESSING_COMPLETE`: The document was successfully processed.  `DOCUMENT_ERROR`: The document could not be processed. Possible causes include: The document was an unacceptable document type such as an offer letter or bank statement, the document image was cropped or blurry, or the document was corrupted.  `UNKNOWN` or `null`: An internal error occured. If this happens repeatedly, contact support or your Plaid account manager. | [optional]
**doc_id** | Option<**String**> | An identifier of the document that is also present in the paystub response. | [optional]
**doc_type** | Option<[**crate::models::DocType**](DocType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


