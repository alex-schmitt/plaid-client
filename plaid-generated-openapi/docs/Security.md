# Security

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**security_id** | **String** | A unique, Plaid-specific identifier for the security, used to associate securities with holdings. Like all Plaid identifiers, the `security_id` is case sensitive. The `security_id` may change if inherent details of the security change due to a corporate action, for example, in the event of a ticker symbol change or CUSIP change. | 
**isin** | Option<**String**> | 12-character ISIN, a globally unique securities identifier. Please note that Plaid's customers must hold a license directly from CUSIP Global Services to receive CUSIP & ISIN data. This field will be null by default for new customers. For existing customers, this field will be null by default starting on Sept 15, 2023. If you would like access to this field, please contact your Plaid Account Manager or reach out to investments-vendors@plaid.com. | 
**cusip** | Option<**String**> | 9-character CUSIP, an identifier assigned to North American securities. Please note that Plaid's customers must hold a license directly from CUSIP Global Services to receive CUSIP & ISIN data. This field will be null by default for new customers. For existing customers, this field will be null by default starting on Sept 15, 2023. If you would like access to this field, please contact your Plaid Account Manager or reach out to investments-vendors@plaid.com. | 
**sedol** | Option<**String**> | 7-character SEDOL, an identifier assigned to securities in the UK. | 
**institution_security_id** | Option<**String**> | An identifier given to the security by the institution | 
**institution_id** | Option<**String**> | If `institution_security_id` is present, this field indicates the Plaid `institution_id` of the institution to whom the identifier belongs. | 
**proxy_security_id** | Option<**String**> | In certain cases, Plaid will provide the ID of another security whose performance resembles this security, typically when the original security has low volume, or when a private security can be modeled with a publicly traded security. | 
**name** | Option<**String**> | A descriptive name for the security, suitable for display. | 
**ticker_symbol** | Option<**String**> | The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available. | 
**is_cash_equivalent** | Option<**bool**> | Indicates that a security is a highly liquid asset and can be treated like cash. | 
**r#type** | Option<**String**> | The security type of the holding. Valid security types are:  `cash`: Cash, currency, and money market funds  `cryptocurrency`: Digital or virtual currencies  `derivative`: Options, warrants, and other derivative instruments  `equity`: Domestic and foreign equities  `etf`: Multi-asset exchange-traded investment funds  `fixed income`: Bonds and certificates of deposit (CDs)  `loan`: Loans and loan receivables  `mutual fund`: Open- and closed-end vehicles pooling funds of multiple investors  `other`: Unknown or other investment types | 
**close_price** | Option<**f64**> | Price of the security at the close of the previous trading session. Null for non-public securities.  If the security is a foreign currency this field will be updated daily and will be priced in USD.  If the security is a cryptocurrency, this field will be updated multiple times a day. As crypto prices can fluctuate quickly and data may become stale sooner than other asset classes, refer to `update_datetime` with the time when the price was last updated.  | 
**close_price_as_of** | Option<[**String**](string.md)> | Date for which `close_price` is accurate. Always `null` if `close_price` is `null`. | 
**update_datetime** | Option<**String**> | Date and time at which `close_price` is accurate, in ISO 8601 format (YYYY-MM-DDTHH:mm:ssZ). Always `null` if `close_price` is `null`. | [optional]
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the price given. Always `null` if `unofficial_currency_code` is non-`null`. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


