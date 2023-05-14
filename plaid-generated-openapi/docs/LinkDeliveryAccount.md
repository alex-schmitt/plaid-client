# LinkDeliveryAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Plaid `account_id` | [optional]
**name** | Option<**String**> | The official account name | [optional]
**mask** | Option<**String**> | The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts. It may also not match the mask that the bank displays to the user. | [optional]
**r#type** | Option<**String**> | The account type. See the [Account schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full list of possible values | [optional]
**subtype** | Option<**String**> | The account subtype. See the [Account schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full list of possible values | [optional]
**verification_status** | Option<[**crate::models::LinkDeliveryVerificationStatus**](LinkDeliveryVerificationStatus.md)> |  | [optional]
**class_type** | Option<**String**> | If micro-deposit verification is being used, indicates whether the account being verified is a `business` or `personal` account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


