# SecurityOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**isin** | Option<**String**> | 12-character ISIN, a globally unique securities identifier. Please note that Plaid's customers must hold a license directly from CUSIP Global Services to receive CUSIP & ISIN data. This field will be null by default for new customers. For existing customers, this field will be null by default starting on Sept 15, 2023. If you would like access to this field, please contact your Plaid Account Manager or reach out to investments-vendors@plaid.com. | [optional]
**cusip** | Option<**String**> | 9-character CUSIP, an identifier assigned to North American securities. Please note that Plaid's customers must hold a license directly from CUSIP Global Services to receive CUSIP & ISIN data. This field will be null by default for new customers. For existing customers, this field will be null by default starting on Sept 15, 2023. If you would like access to this field, please contact your Plaid Account Manager or reach out to investments-vendors@plaid.com. | [optional]
**sedol** | Option<**String**> | 7-character SEDOL, an identifier assigned to securities in the UK. | [optional]
**name** | Option<**String**> | A descriptive name for the security, suitable for display. | [optional]
**ticker_symbol** | Option<**String**> | The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available. | [optional]
**currency** | Option<**String**> | Either a valid `iso_currency_code` or `unofficial_currency_code` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


