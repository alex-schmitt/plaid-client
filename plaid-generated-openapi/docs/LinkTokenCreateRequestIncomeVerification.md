# LinkTokenCreateRequestIncomeVerification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**income_verification_id** | Option<**String**> | The `income_verification_id` of the verification instance, as provided by `/income/verification/create`. | [optional]
**asset_report_id** | Option<**String**> | The `asset_report_id` of an asset report associated with the user, as provided by `/asset_report/create`. Providing an `asset_report_id` is optional and can be used to verify the user through a streamlined flow. If provided, the bank linking flow will be skipped. | [optional]
**precheck_id** | Option<**String**> | The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow by streamlining the Link interface presented to the end user. | [optional]
**access_tokens** | Option<**Vec<String>**> | An array of access tokens corresponding to Items that a user has previously connected with. Data from these institutions will be cross-referenced with document data received during the Document Income flow to help verify that the uploaded documents are accurate. If the `transactions` product was not initialized for these Items during link, it will be initialized after this Link session.  This field should only be used with the `payroll` income source type. | [optional]
**income_source_types** | Option<[**Vec<crate::models::IncomeVerificationSourceType>**](IncomeVerificationSourceType.md)> | The types of source income data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options. | [optional]
**bank_income** | Option<[**crate::models::LinkTokenCreateRequestIncomeVerificationBankIncome**](LinkTokenCreateRequestIncomeVerificationBankIncome.md)> |  | [optional]
**payroll_income** | Option<[**crate::models::LinkTokenCreateRequestIncomeVerificationPayrollIncome**](LinkTokenCreateRequestIncomeVerificationPayrollIncome.md)> |  | [optional]
**stated_income_sources** | Option<[**Vec<crate::models::LinkTokenCreateRequestUserStatedIncomeSource>**](LinkTokenCreateRequestUserStatedIncomeSource.md)> | A list of user stated income sources | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


