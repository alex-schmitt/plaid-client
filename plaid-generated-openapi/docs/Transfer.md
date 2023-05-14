# Transfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Plaid’s unique identifier for a transfer. | 
**ach_class** | Option<[**crate::models::AchClass**](ACHClass.md)> |  | [optional]
**account_id** | Option<**String**> | The Plaid `account_id` corresponding to the end-user account that will be debited or credited. | [optional]
**funding_account_id** | **String** | The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited. | 
**r#type** | [**crate::models::TransferType**](TransferType.md) |  | 
**user** | [**crate::models::TransferUserInResponse**](TransferUserInResponse.md) |  | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**description** | **String** | The description of the transfer. | 
**created** | **String** | The datetime when this transfer was created. This will be of the form `2006-01-02T15:04:05Z` | 
**status** | [**crate::models::TransferStatus**](TransferStatus.md) |  | 
**sweep_status** | Option<[**crate::models::TransferSweepStatus**](TransferSweepStatus.md)> |  | [optional]
**network** | [**crate::models::TransferNetwork**](TransferNetwork.md) |  | 
**cancellable** | **bool** | When `true`, you can still cancel this transfer. | 
**failure_reason** | Option<[**crate::models::TransferFailure**](TransferFailure.md)> |  | 
**metadata** | Option<**::std::collections::HashMap<String, String>**> | The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: The JSON values must be Strings (no nested JSON objects allowed) Only ASCII characters may be used Maximum of 50 key/value pairs Maximum key length of 40 characters Maximum value length of 500 characters  | 
**origination_account_id** | **String** | Plaid’s unique identifier for the origination account that was used for this transfer. | 
**guarantee_decision** | Option<[**crate::models::TransferAuthorizationGuaranteeDecision**](TransferAuthorizationGuaranteeDecision.md)> |  | 
**guarantee_decision_rationale** | Option<[**crate::models::TransferAuthorizationGuaranteeDecisionRationale**](TransferAuthorizationGuaranteeDecisionRationale.md)> |  | 
**iso_currency_code** | **String** | The currency of the transfer amount, e.g. \"USD\" | 
**standard_return_window** | Option<[**String**](string.md)> | The date 3 business days from settlement date indicating the following ACH returns can no longer happen: R01, R02, R03, R29. This will be of the form YYYY-MM-DD. | 
**unauthorized_return_window** | Option<[**String**](string.md)> | The date 61 business days from settlement date indicating the following ACH returns can no longer happen: R05, R07, R10, R11, R51, R33, R37, R38, R51, R52, R53. This will be of the form YYYY-MM-DD. | 
**expected_settlement_date** | Option<[**String**](string.md)> | An estimation of the settlement date which can be useful when the transfer is `pending`. Only set for ACH transfers and is `null` for non-ACH transfers. This will be of the form YYYY-MM-DD. | 
**originator_client_id** | Option<**String**> | The Plaid client ID that is the originator of this transfer. Only present if created on behalf of another client as a third-party sender (TPS). | 
**refunds** | [**Vec<crate::models::TransferRefund>**](TransferRefund.md) | A list of refunds associated with this transfer. | 
**recurring_transfer_id** | Option<**String**> | The id of the recurring transfer if this transfer belongs to a recurring transfer. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


