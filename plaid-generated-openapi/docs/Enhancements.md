# Enhancements

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_name** | Option<**String**> | The name of the primary counterparty, such as the merchant or the financial institution, as extracted by Plaid from the raw description. | [optional]
**website** | Option<**String**> | The website associated with this transaction, if available. | [optional]
**logo_url** | Option<**String**> | The URL of a logo associated with this transaction, if available. The logo is formatted as a 100x100 pixel PNG file. | [optional]
**check_number** | Option<**String**> | The check number of the transaction. This field is only populated for check transactions. | [optional]
**payment_channel** | [**crate::models::PaymentChannel**](PaymentChannel.md) |  | 
**category_id** | Option<**String**> | The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget). | 
**category** | **Vec<String>** | A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget). | 
**location** | [**crate::models::Location**](Location.md) |  | 
**personal_finance_category** | Option<[**crate::models::PersonalFinanceCategory**](PersonalFinanceCategory.md)> |  | [optional]
**personal_finance_category_icon_url** | Option<**String**> | A link to the icon associated with the primary personal finance category. The logo will always be 100x100 pixels. | [optional]
**counterparties** | Option<[**Vec<crate::models::Counterparty>**](Counterparty.md)> | The counterparties present in the transaction. Counterparties, such as the merchant or the financial institution, are extracted by Plaid from the raw description. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


