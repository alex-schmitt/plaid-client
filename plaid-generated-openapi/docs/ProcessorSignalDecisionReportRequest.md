# ProcessorSignalDecisionReportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**processor_token** | **String** | The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>` | 
**client_transaction_id** | **String** | Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate` | 
**initiated** | **bool** | `true` if the ACH transaction was initiated, `false` otherwise.  This field must be returned as a boolean. If formatted incorrectly, this will result in an [`INVALID_FIELD`](/docs/errors/invalid-request/#invalid_field) error. | 
**days_funds_on_hold** | Option<**i32**> | The actual number of days (hold time) since the ACH debit transaction that you wait before making funds available to your customers. The holding time could affect the ACH return rate.  For example, use 0 if you make funds available to your customers instantly or the same day following the debit transaction, or 1 if you make funds available the next day following the debit initialization. | [optional]
**decision_outcome** | Option<[**crate::models::SignalDecisionOutcome**](SignalDecisionOutcome.md)> |  | [optional]
**payment_method** | Option<[**crate::models::SignalPaymentMethod**](SignalPaymentMethod.md)> |  | [optional]
**amount_instantly_available** | Option<**f64**> | The amount (in USD) made available to your customers instantly following the debit transaction. It could be a partial amount of the requested transaction (example: 102.05). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


