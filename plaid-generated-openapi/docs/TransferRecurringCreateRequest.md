# TransferRecurringCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**access_token** | **String** | The Plaid `access_token` for the account that will be debited or credited. Required if not using `payment_profile_token`. | 
**idempotency_key** | **String** | A random key provided by the client, per unique recurring transfer. Maximum of 50 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a recurring fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single recurring transfer is created. | 
**account_id** | **String** | The Plaid `account_id` corresponding to the end-user account that will be debited or credited. Returned only if `account_id` was set on intent creation. | 
**funding_account_id** | Option<**String**> | The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited. Defaults to the account configured during onboarding. | [optional]
**r#type** | [**crate::models::TransferType**](TransferType.md) |  | 
**network** | [**crate::models::TransferNetwork**](TransferNetwork.md) |  | 
**ach_class** | Option<[**crate::models::AchClass**](ACHClass.md)> |  | [optional]
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**user_present** | Option<**bool**> | If the end user is initiating the specific transfer themselves via an interactive UI, this should be `true`; for automatic recurring payments where the end user is not actually initiating each individual transfer, it should be `false`. | 
**iso_currency_code** | Option<**String**> | The currency of the transfer amount. The default value is \"USD\". | [optional]
**description** | **String** | The description of the recurring transfer. | 
**test_clock_id** | Option<**String**> | Plaid’s unique identifier for a test clock. | [optional]
**schedule** | [**crate::models::TransferRecurringSchedule**](TransferRecurringSchedule.md) |  | 
**user** | [**crate::models::TransferUserInRequest**](TransferUserInRequest.md) |  | 
**device** | [**crate::models::TransferDevice**](TransferDevice.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


