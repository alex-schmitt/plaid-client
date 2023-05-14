# CreditBankEmploymentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | **String** | The unique identifier for the Item. | 
**last_updated_time** | **String** | The time when this Item's data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. \"2018-04-12T03:32:11Z\"). | 
**institution_id** | **String** | The unique identifier of the institution associated with the Item. | 
**institution_name** | **String** | The name of the institution associated with the Item. | 
**bank_employments** | [**Vec<crate::models::CreditBankEmployment>**](CreditBankEmployment.md) | The bank employment information for this Item. Each entry in the array is a different employer found. | 
**bank_employment_accounts** | [**Vec<crate::models::CreditBankIncomeAccount>**](CreditBankIncomeAccount.md) | The Item's accounts that have Bank Employment data. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


