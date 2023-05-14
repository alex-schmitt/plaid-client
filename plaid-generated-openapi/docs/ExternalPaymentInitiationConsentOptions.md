# ExternalPaymentInitiationConsentOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_refund_details** | Option<**bool**> | When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response. | [optional]
**iban** | Option<**String**> | The International Bank Account Number (IBAN) for the payer's account. Where possible, the end user will be able to set up payment consent using only the specified bank account if provided. | [optional]
**bacs** | Option<[**crate::models::PaymentInitiationOptionalRestrictionBacs**](PaymentInitiationOptionalRestrictionBacs.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


