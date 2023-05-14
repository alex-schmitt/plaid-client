# FdxNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**notification_id** | **String** | Id of notification | 
**r#type** | [**crate::models::FdxNotificationType**](FDXNotificationType.md) |  | 
**sent_on** | **String** | ISO 8601 date-time in format 'YYYY-MM-DDThh:mm:ss.nnn[Z|[+|-]hh:mm]' according to [IETF RFC3339](https://xml2rfc.tools.ietf.org/public/rfc/html/rfc3339.html#anchor14) | 
**category** | [**crate::models::FdxNotificationCategory**](FDXNotificationCategory.md) |  | 
**severity** | Option<[**crate::models::FdxNotificationSeverity**](FDXNotificationSeverity.md)> |  | [optional]
**priority** | Option<[**crate::models::FdxNotificationPriority**](FDXNotificationPriority.md)> |  | [optional]
**publisher** | [**crate::models::FdxParty**](FDXParty.md) |  | 
**subscriber** | Option<[**crate::models::FdxParty**](FDXParty.md)> |  | [optional]
**notification_payload** | [**crate::models::FdxNotificationPayload**](FDXNotificationPayload.md) |  | 
**url** | Option<[**crate::models::FdxHateoasLink**](FDXHateoasLink.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


