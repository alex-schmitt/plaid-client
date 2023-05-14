# CreditPayrollIncomePrecheckRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**user_token** | Option<**String**> | The user token associated with the User data is being requested for. | [optional]
**access_tokens** | Option<**Vec<String>**> | An array of access tokens corresponding to Items belonging to the user whose eligibility is being checked. Note that if the Items specified here are not already initialized with `transactions`, providing them in this field will cause these Items to be initialized with (and billed for) the Transactions product. | [optional]
**employer** | Option<[**crate::models::IncomeVerificationPrecheckEmployer**](IncomeVerificationPrecheckEmployer.md)> |  | [optional]
**us_military_info** | Option<[**crate::models::IncomeVerificationPrecheckMilitaryInfo**](IncomeVerificationPrecheckMilitaryInfo.md)> |  | [optional]
**payroll_institution** | Option<[**crate::models::IncomeVerificationPrecheckPayrollInstitution**](IncomeVerificationPrecheckPayrollInstitution.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


