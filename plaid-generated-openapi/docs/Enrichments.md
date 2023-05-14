# Enrichments

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**check_number** | Option<**String**> | The check number of the transaction. This field is only populated for check transactions. | [optional]
**counterparties** | [**Vec<crate::models::Counterparty>**](Counterparty.md) | The counterparties present in the transaction. Counterparties, such as the merchant or the financial institution, are extracted by Plaid from the raw description. | 
**entity_id** | Option<**String**> | A unique, stable, Plaid-generated id that maps to the primary counterparty. | [optional]
**legacy_category_id** | Option<**String**> | The ID of the legacy category to which this transaction belongs. For a full list of legacy categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  We recommend using the `personal_finance_category` for transaction categorization to obtain the best results. | [optional]
**legacy_category** | Option<**Vec<String>**> | A hierarchical array of the legacy categories to which this transaction belongs. For a full list of legacy categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  We recommend using the `personal_finance_category` for transaction categorization to obtain the best results. | [optional]
**location** | [**crate::models::Location**](Location.md) |  | 
**logo_url** | Option<**String**> | The URL of a logo associated with this transaction, if available. The logo is formatted as a 100x100 pixel PNG file. | 
**merchant_name** | Option<**String**> | The name of the primary counterparty, such as the merchant or the financial institution, as extracted by Plaid from the raw description. | 
**payment_channel** | [**crate::models::PaymentChannel**](PaymentChannel.md) |  | 
**personal_finance_category** | Option<[**crate::models::PersonalFinanceCategory**](PersonalFinanceCategory.md)> |  | 
**personal_finance_category_icon_url** | **String** | A link to the icon associated with the primary personal finance category. The logo will always be 100x100 pixels. | 
**website** | Option<**String**> | The website associated with this transaction. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


