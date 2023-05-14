/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PlaidError : We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct PlaidError {
    #[serde(rename = "error_type")]
    pub error_type: crate::models::PlaidErrorType,
    /// The particular error code. Safe for programmatic use.
    #[serde(rename = "error_code")]
    pub error_code: String,
    /// A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    #[serde(rename = "error_message")]
    pub error_message: String,
    /// A user-friendly representation of the error code. `null` if the error is not related to user action.  This may change over time and is not safe for programmatic use.
    #[serde(rename = "display_message", deserialize_with = "Option::deserialize")]
    pub display_message: Option<String>,
    /// A unique ID identifying the request, to be used for troubleshooting purposes. This field will be omitted in errors provided by webhooks.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The HTTP status code associated with the error. This will only be returned in the response body when the error information is provided via a webhook.
    #[serde(
        rename = "status",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<Option<f32>>,
    /// The URL of a Plaid documentation page with more information about the error
    #[serde(rename = "documentation_url", skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// Suggested steps for resolving the error
    #[serde(
        rename = "suggested_action",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suggested_action: Option<Option<String>>,
}

impl PlaidError {
    /// We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    pub fn new(
        error_type: crate::models::PlaidErrorType,
        error_code: String,
        error_message: String,
        display_message: Option<String>,
    ) -> PlaidError {
        PlaidError {
            error_type,
            error_code,
            error_message,
            display_message,
            request_id: None,
            status: None,
            documentation_url: None,
            suggested_action: None,
        }
    }
}