# CreditW2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_metadata** | [**crate::models::CreditDocumentMetadata**](CreditDocumentMetadata.md) |  | 
**document_id** | **String** | An identifier of the document referenced by the document metadata. | 
**employer** | [**crate::models::CreditPayStubEmployer**](CreditPayStubEmployer.md) |  | 
**employee** | [**crate::models::CreditPayStubEmployee**](CreditPayStubEmployee.md) |  | 
**tax_year** | Option<**String**> | The tax year of the W2 document. | 
**employer_id_number** | Option<**String**> | An employee identification number or EIN. | 
**wages_tips_other_comp** | Option<**String**> | Wages from tips and other compensation. | 
**federal_income_tax_withheld** | Option<**String**> | Federal income tax withheld for the tax year. | 
**social_security_wages** | Option<**String**> | Wages from social security. | 
**social_security_tax_withheld** | Option<**String**> | Social security tax withheld for the tax year. | 
**medicare_wages_and_tips** | Option<**String**> | Wages and tips from medicare. | 
**medicare_tax_withheld** | Option<**String**> | Medicare tax withheld for the tax year. | 
**social_security_tips** | Option<**String**> | Tips from social security. | 
**allocated_tips** | Option<**String**> | Allocated tips. | 
**box_9** | Option<**String**> | Contents from box 9 on the W2. | 
**dependent_care_benefits** | Option<**String**> | Dependent care benefits. | 
**nonqualified_plans** | Option<**String**> | Nonqualified plans. | 
**box_12** | [**Vec<crate::models::W2Box12>**](W2Box12.md) |  | 
**statutory_employee** | Option<**String**> | Statutory employee. | 
**retirement_plan** | Option<**String**> | Retirement plan. | 
**third_party_sick_pay** | Option<**String**> | Third party sick pay. | 
**other** | Option<**String**> | Other. | 
**state_and_local_wages** | [**Vec<crate::models::W2StateAndLocalWages>**](W2StateAndLocalWages.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


