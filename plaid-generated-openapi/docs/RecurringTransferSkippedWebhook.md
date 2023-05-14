# RecurringTransferSkippedWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `TRANSFER` | 
**webhook_code** | **String** | `RECURRING_TRANSFER_SKIPPED` | 
**recurring_transfer_id** | **String** | Plaid’s unique identifier for a recurring transfer. | 
**authorization_decision** | [**crate::models::TransferAuthorizationDecision**](TransferAuthorizationDecision.md) |  | 
**authorization_decision_rationale_code** | Option<[**crate::models::TransferAuthorizationDecisionRationaleCode**](TransferAuthorizationDecisionRationaleCode.md)> |  | [optional]
**skipped_origination_date** | [**String**](string.md) | The planned date on which Plaid is unable to originate a new ACH transaction of the recurring transfer. This will be of the form YYYY-MM-DD. | 
**environment** | [**crate::models::WebhookEnvironmentValues**](WebhookEnvironmentValues.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


