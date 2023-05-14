# LinkTokenCreateRequestAuth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_type_select_enabled** | Option<**bool**> | Specifies whether Auth Type Select is enabled for the Link session, allowing the end user to choose between linking instantly or manually prior to selecting their financial institution. Note that this can only be true if `same_day_microdeposits_enabled` is set to true. | [optional][default to false]
**automated_microdeposits_enabled** | Option<**bool**> | Specifies whether the Link session is enabled for the Automated Micro-deposits flow. | [optional]
**instant_match_enabled** | Option<**bool**> | Specifies whether the Link session is enabled for the Instant Match flow. As of November 2022, Instant Match will be enabled by default. Instant Match can be disabled by setting this field to `false`. | [optional]
**same_day_microdeposits_enabled** | Option<**bool**> | Specifies whether the Link session is enabled for the Same Day Micro-deposits flow. | [optional]
**flow_type** | Option<**String**> | This field has been deprecated in favor of `auth_type_select_enabled`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


