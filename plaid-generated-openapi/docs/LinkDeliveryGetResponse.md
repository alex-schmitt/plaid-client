# LinkDeliveryGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**crate::models::LinkDeliverySessionStatus**](LinkDeliverySessionStatus.md) |  | 
**created_at** | **String** | Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Link Delivery Session was created at. | 
**completed_at** | Option<**String**> | Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Link Delivery Session was completed at. | [optional]
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 
**access_tokens** | Option<**Vec<String>**> | An array of access tokens associated with the Link Delivery session. | [optional]
**item_ids** | Option<**Vec<String>**> | An array of `item_id`s associated with the Link Delivery session. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


