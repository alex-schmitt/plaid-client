# WatchlistScreeningEntityProgramGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the associated entity program. | 
**created_at** | **String** | An ISO8601 formatted timestamp. | 
**is_rescanning_enabled** | **bool** | Indicator specifying whether the program is enabled and will perform daily rescans. | 
**lists_enabled** | [**Vec<crate::models::EntityWatchlistCode>**](EntityWatchlistCode.md) | Watchlists enabled for the associated program | 
**name** | **String** | A name for the entity program to define its purpose. For example, \"High Risk Organizations\" or \"Applicants\". | 
**name_sensitivity** | [**crate::models::ProgramNameSensitivity**](ProgramNameSensitivity.md) |  | 
**audit_trail** | [**crate::models::WatchlistScreeningAuditTrail**](WatchlistScreeningAuditTrail.md) |  | 
**is_archived** | **bool** | Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring. | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


