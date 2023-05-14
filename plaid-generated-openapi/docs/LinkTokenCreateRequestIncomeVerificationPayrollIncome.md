# LinkTokenCreateRequestIncomeVerificationPayrollIncome

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_types** | Option<[**Vec<crate::models::IncomeVerificationPayrollFlowType>**](IncomeVerificationPayrollFlowType.md)> | The types of payroll income verification to enable for the Link session. If none are specified, then users will see both document and digital payroll income. | [optional]
**is_update_mode** | Option<**bool**> | An identifier to indicate whether the income verification Link token will be used for an update or not | [optional][default to false]
**item_id_to_update** | Option<**String**> | Uniquely identify a payroll income item to update with. It should only be used for update mode. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


