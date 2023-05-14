# RiskCheckEmail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_deliverable** | [**crate::models::RiskCheckEmailIsDeliverableStatus**](RiskCheckEmailIsDeliverableStatus.md) |  | 
**breach_count** | Option<**i32**> | Count of all known breaches of this email address if known. | 
**first_breached_at** | Option<[**String**](string.md)> | A date in the format YYYY-MM-DD (RFC 3339 Section 5.6). | 
**last_breached_at** | Option<[**String**](string.md)> | A date in the format YYYY-MM-DD (RFC 3339 Section 5.6). | 
**domain_registered_at** | Option<[**String**](string.md)> | A date in the format YYYY-MM-DD (RFC 3339 Section 5.6). | 
**domain_is_free_provider** | [**crate::models::RiskCheckEmailDomainIsFreeProvider**](RiskCheckEmailDomainIsFreeProvider.md) |  | 
**domain_is_custom** | [**crate::models::RiskCheckEmailDomainIsCustom**](RiskCheckEmailDomainIsCustom.md) |  | 
**domain_is_disposable** | [**crate::models::RiskCheckEmailDomainIsDisposable**](RiskCheckEmailDomainIsDisposable.md) |  | 
**top_level_domain_is_suspicious** | [**crate::models::RiskCheckEmailTopLevelDomainIsSuspicious**](RiskCheckEmailTopLevelDomainIsSuspicious.md) |  | 
**linked_services** | [**Vec<crate::models::RiskCheckLinkedService>**](RiskCheckLinkedService.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


