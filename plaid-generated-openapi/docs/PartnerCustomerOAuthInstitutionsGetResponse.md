# PartnerCustomerOAuthInstitutionsGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flowdown_status** | Option<[**crate::models::PartnerEndCustomerFlowdownStatus**](PartnerEndCustomerFlowdownStatus.md)> |  | [optional]
**questionnaire_status** | Option<[**crate::models::PartnerEndCustomerQuestionnaireStatus**](PartnerEndCustomerQuestionnaireStatus.md)> |  | [optional]
**institutions** | Option<[**Vec<crate::models::PartnerEndCustomerOAuthInstitution>**](PartnerEndCustomerOAuthInstitution.md)> | The OAuth institutions with which the end customer's application is being registered. | [optional]
**request_id** | Option<**String**> | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


