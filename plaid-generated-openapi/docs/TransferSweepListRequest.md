# TransferSweepListRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**start_date** | Option<**String**> | The start datetime of sweeps to return (RFC 3339 format). | [optional]
**end_date** | Option<**String**> | The end datetime of sweeps to return (RFC 3339 format). | [optional]
**count** | Option<**i32**> | The maximum number of sweeps to return. | [optional][default to 25]
**offset** | Option<**i32**> | The number of sweeps to skip before returning results. | [optional][default to 0]
**originator_client_id** | Option<**String**> | Filter sweeps to only those with the specified originator client. | [optional]
**funding_account_id** | Option<**String**> | Filter sweeps to only those with the specified `funding_account_id`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


