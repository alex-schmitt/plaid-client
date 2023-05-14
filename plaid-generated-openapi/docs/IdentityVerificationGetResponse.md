# IdentityVerificationGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the associated Identity Verification attempt. | 
**client_user_id** | **String** | An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object. | 
**created_at** | **String** | An ISO8601 formatted timestamp. | 
**completed_at** | Option<**String**> | An ISO8601 formatted timestamp. | 
**previous_attempt_id** | Option<**String**> | The ID for the Identity Verification preceding this session. This field will only be filled if the current Identity Verification is a retry of a previous attempt. | 
**shareable_url** | Option<**String**> | A shareable URL that can be sent directly to the user to complete verification | 
**template** | [**crate::models::IdentityVerificationTemplateReference**](IdentityVerificationTemplateReference.md) |  | 
**user** | [**crate::models::IdentityVerificationUserData**](IdentityVerificationUserData.md) |  | 
**status** | [**crate::models::IdentityVerificationStatus**](IdentityVerificationStatus.md) |  | 
**steps** | [**crate::models::IdentityVerificationStepSummary**](IdentityVerificationStepSummary.md) |  | 
**documentary_verification** | Option<[**crate::models::DocumentaryVerification**](DocumentaryVerification.md)> |  | 
**kyc_check** | Option<[**crate::models::KycCheckDetails**](KYCCheckDetails.md)> |  | 
**risk_check** | Option<[**crate::models::RiskCheckDetails**](RiskCheckDetails.md)> |  | 
**watchlist_screening_id** | Option<**String**> | ID of the associated screening. | 
**redacted_at** | Option<**String**> | An ISO8601 formatted timestamp. | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


