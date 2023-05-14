# TransferAuthorizationCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**access_token** | Option<**String**> | The Plaid `access_token` for the account that will be debited or credited. Required if not using `payment_profile_token`. | [optional]
**account_id** | Option<**String**> | The Plaid `account_id` corresponding to the end-user account that will be debited or credited. Returned only if `account_id` was set on intent creation. | [optional]
**funding_account_id** | Option<**String**> | The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited. Defaults to the account configured during onboarding. | [optional]
**payment_profile_token** | Option<**String**> | The payment profile token associated with the Payment Profile that will be debited or credited. Required if not using `access_token`. | [optional]
**r#type** | [**crate::models::TransferType**](TransferType.md) |  | 
**network** | [**crate::models::TransferNetwork**](TransferNetwork.md) |  | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**ach_class** | Option<[**crate::models::AchClass**](ACHClass.md)> |  | [optional]
**user** | [**crate::models::TransferAuthorizationUserInRequest**](TransferAuthorizationUserInRequest.md) |  | 
**device** | Option<[**crate::models::TransferAuthorizationDevice**](TransferAuthorizationDevice.md)> |  | [optional]
**origination_account_id** | Option<**String**> | Plaid's unique identifier for the origination account for this authorization. If not specified, the default account will be used. | [optional]
**iso_currency_code** | Option<**String**> | The currency of the transfer amount. The default value is \"USD\". | [optional]
**idempotency_key** | Option<**String**> | A random key provided by the client, per unique authorization. Maximum of 50 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create an authorization fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single authorization is created.  Failure to provide this key may result in duplicate charges.  Required for guaranteed ACH customers. | [optional]
**user_present** | Option<**bool**> | Required for Guarantee. If the end user is initiating the specific transfer themselves via an interactive UI, this should be `true`; for automatic recurring payments where the end user is not actually initiating each individual transfer, it should be `false`. | [optional]
**with_guarantee** | Option<**bool**> | If set to `false`, Plaid will not offer a `guarantee_decision` for this request(Guarantee customers only). | [optional][default to true]
**beacon_session_id** | Option<**String**> | The unique identifier returned by Plaid's [beacon](https://plaid.com/docs/transfer/guarantee/#using-a-beacon) when it is run on your webpage. Required for Guarantee customers who are not using [Transfer UI](https://plaid.com/docs/transfer/using-transfer-ui/) and have a web checkout experience. | [optional]
**originator_client_id** | Option<**String**> | The Plaid client ID that is the originator of this transfer. Only needed if creating transfers on behalf of another client as a third-party sender (TPS). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


