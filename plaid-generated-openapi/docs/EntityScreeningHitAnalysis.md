# EntityScreeningHitAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**documents** | Option<[**crate::models::MatchSummaryCode**](MatchSummaryCode.md)> |  | [optional]
**email_addresses** | Option<[**crate::models::MatchSummaryCode**](MatchSummaryCode.md)> |  | [optional]
**locations** | Option<[**crate::models::MatchSummaryCode**](MatchSummaryCode.md)> |  | [optional]
**names** | Option<[**crate::models::MatchSummaryCode**](MatchSummaryCode.md)> |  | [optional]
**phone_numbers** | Option<[**crate::models::MatchSummaryCode**](MatchSummaryCode.md)> |  | [optional]
**urls** | Option<[**crate::models::MatchSummaryCode**](MatchSummaryCode.md)> |  | [optional]
**search_terms_version** | **f32** | The version of the entity screening's `search_terms` that were compared when the entity screening hit was added. entity screening hits are immutable once they have been reviewed. If changes are detected due to updates to the entity screening's `search_terms`, the associated entity program, or the list's source data prior to review, the entity screening hit will be updated to reflect those changes. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


