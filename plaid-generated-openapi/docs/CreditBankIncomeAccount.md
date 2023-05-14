# CreditBankIncomeAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | Plaid's unique identifier for the account. | 
**mask** | Option<**String**> | The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user. | 
**name** | **String** | The name of the bank account. | 
**official_name** | Option<**String**> | The official name of the bank account. | 
**subtype** | [**crate::models::DepositoryAccountSubtype**](DepositoryAccountSubtype.md) |  | 
**r#type** | [**crate::models::CreditBankIncomeAccountType**](CreditBankIncomeAccountType.md) |  | 
**owners** | [**Vec<crate::models::Owner>**](Owner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


