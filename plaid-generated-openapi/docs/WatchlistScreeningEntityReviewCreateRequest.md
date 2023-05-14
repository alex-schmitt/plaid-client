# WatchlistScreeningEntityReviewCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**confirmed_hits** | **Vec<String>** | Hits to mark as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected. | 
**dismissed_hits** | **Vec<String>** | Hits to mark as a false positive after thorough manual review. These hits will never recur or be updated once dismissed. | 
**comment** | Option<**String**> | A comment submitted by a team member as part of reviewing a watchlist screening. | [optional]
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**entity_watchlist_screening_id** | **String** | ID of the associated entity screening. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


