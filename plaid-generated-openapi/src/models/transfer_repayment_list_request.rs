/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferRepaymentListRequest : Defines the request schema for `/transfer/repayment/list`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferRepaymentListRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The start datetime of repayments to return (RFC 3339 format).
    #[serde(
        rename = "start_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date: Option<Option<String>>,
    /// The end datetime of repayments to return (RFC 3339 format).
    #[serde(
        rename = "end_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date: Option<Option<String>>,
    /// The maximum number of repayments to return.
    #[serde(
        rename = "count",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<Option<i32>>,
    /// The number of repayments to skip before returning results.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl TransferRepaymentListRequest {
    /// Defines the request schema for `/transfer/repayment/list`
    pub fn new() -> TransferRepaymentListRequest {
        TransferRepaymentListRequest {
            client_id: None,
            secret: None,
            start_date: None,
            end_date: None,
            count: None,
            offset: None,
        }
    }
}
