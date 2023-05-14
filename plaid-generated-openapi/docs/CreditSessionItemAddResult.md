# CreditSessionItemAddResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_token** | Option<**String**> | Returned once a user has successfully linked their Item. | [optional]
**item_id** | Option<**String**> | The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive. | [optional]
**institution_id** | Option<**String**> | The Plaid Institution ID associated with the Item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


