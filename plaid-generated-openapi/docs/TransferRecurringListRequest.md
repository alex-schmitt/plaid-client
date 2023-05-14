# TransferRecurringListRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**start_time** | Option<**String**> | The start datetime of recurring transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`) | [optional]
**end_time** | Option<**String**> | The end datetime of recurring transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`) | [optional]
**count** | Option<**i32**> | The maximum number of recurring transfers to return. | [optional][default to 25]
**offset** | Option<**i32**> | The number of recurring transfers to skip before returning results. | [optional][default to 0]
**funding_account_id** | Option<**String**> | Filter recurring transfers to only those with the specified `funding_account_id`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


