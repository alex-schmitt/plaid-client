# CreditEmploymentVerification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | ID of the payroll provider account. | 
**status** | Option<**String**> | Current employment status. | 
**start_date** | Option<[**String**](string.md)> | Start of employment in ISO 8601 format (YYYY-MM-DD). | 
**end_date** | Option<[**String**](string.md)> | End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD). | 
**employer** | [**crate::models::CreditEmployerVerification**](CreditEmployerVerification.md) |  | 
**title** | Option<**String**> | Current title of employee. | 
**platform_ids** | [**crate::models::CreditPlatformIds**](CreditPlatformIds.md) |  | 
**employee_type** | Option<**String**> | The type of employment for the individual. `\"FULL_TIME\"`: A full-time employee. `\"PART_TIME\"`: A part-time employee. `\"CONTRACTOR\"`: An employee typically hired externally through a contracting group. `\"TEMPORARY\"`: A temporary employee. `\"OTHER\"`: The employee type is not one of the above defined types. | 
**last_paystub_date** | Option<[**String**](string.md)> | The date of the employee's most recent paystub in ISO 8601 format (YYYY-MM-DD). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


