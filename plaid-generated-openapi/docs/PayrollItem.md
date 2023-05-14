# PayrollItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | **String** | The `item_id` of the Item associated with this webhook, warning, or error | 
**institution_id** | **String** | The unique identifier of the institution associated with the Item. | 
**institution_name** | **String** | The name of the institution associated with the Item. | 
**accounts** | [**Vec<crate::models::PayrollIncomeAccountData>**](PayrollIncomeAccountData.md) |  | 
**payroll_income** | [**Vec<crate::models::PayrollIncomeObject>**](PayrollIncomeObject.md) |  | 
**status** | Option<[**crate::models::PayrollItemStatus**](PayrollItemStatus.md)> |  | 
**updated_at** | Option<**String**> | Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the Item was updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


