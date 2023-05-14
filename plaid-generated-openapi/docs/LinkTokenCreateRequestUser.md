# LinkTokenCreateRequestUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_user_id** | **String** | A unique ID representing the end user. Typically this will be a user ID number from your application. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`. It is currently used as a means of searching logs for the given user in the Plaid Dashboard. | 
**legal_name** | Option<**String**> | The user's full legal name, used for [micro-deposit based verification flows](https://plaid.com/docs/auth/coverage/). For a small number of customers on legacy flows, providing this field is required to enable micro-deposit-based flows. For all other customers, this field is optional, but providing the user's name in this field when using micro-deposit-based verification will enable certain risk checks and can reduce micro-deposit fraud. | [optional]
**name** | Option<[**crate::models::LinkTokenCreateRequestUserName**](LinkTokenCreateRequestUser_name.md)> |  | [optional]
**phone_number** | Option<**String**> | The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. This field is optional, but required to enable the [returning user experience](https://plaid.com/docs/link/returning-user). | [optional]
**phone_number_verified_time** | Option<**String**> | The date and time the phone number was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This was previously an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user). This field is no longer required to enable the returning user experience.   Only pass a verification time for a phone number that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.   Example: `2020-01-01T00:00:00Z`  | [optional]
**email_address** | Option<**String**> | The user's email address. This field is optional, but required to enable the [pre-authenticated returning user flow](https://plaid.com/docs/link/returning-user/#pre-authenticated-rux). | [optional]
**email_address_verified_time** | Option<**String**> | The date and time the email address was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This was previously an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user). This field is no longer required to enable the returning user experience.   Only pass a verification time for an email address that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.   Example: `2020-01-01T00:00:00Z` | [optional]
**ssn** | Option<**String**> | To be provided in the format \"ddd-dd-dddd\". Not currently used. | [optional]
**date_of_birth** | Option<[**String**](string.md)> | To be provided in the format \"yyyy-mm-dd\". Not currently used. | [optional]
**address** | Option<[**crate::models::UserAddress**](UserAddress.md)> |  | [optional]
**id_number** | Option<[**crate::models::UserIdNumber**](UserIDNumber.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


