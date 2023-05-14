/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferCreateRequest : Defines the request schema for `/transfer/create`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Deprecated. `authorization_id` is now used as idempotency instead.  A random key provided by the client, per unique transfer. Maximum of 50 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single transfer is created.
    #[serde(rename = "idempotency_key", skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// The Plaid `access_token` for the account that will be debited or credited. Required if not using `payment_profile_token`.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// The Plaid `account_id` corresponding to the end-user account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The payment profile token associated with the Payment Profile that will be debited or credited. Required if not using `access_token`.
    #[serde(
        rename = "payment_profile_token",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_profile_token: Option<String>,
    /// Plaid’s unique identifier for a transfer authorization. This parameter also serves the purpose of acting as an idempotency identifier.
    #[serde(rename = "authorization_id")]
    pub authorization_id: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::TransferType>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::models::TransferNetwork>,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The transfer description. Maximum of 10 characters. If reprocessing a returned transfer, please note that the `description` field must be `\"Retry\"` to indicate that it's a retry of a previously returned transfer. You may retry a transfer up to 2 times, within 180 days of creating the original transfer. Only transfers that were returned with code `R01` or `R09` may be retried. For a full listing of ACH return codes, see [Transfer errors](https://plaid.com/docs/errors/transfer/#ach-return-codes).
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "ach_class", skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<crate::models::AchClass>,
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<crate::models::TransferUserInRequestDeprecated>>,
    /// The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: The JSON values must be Strings (no nested JSON objects allowed) Only ASCII characters may be used Maximum of 50 key/value pairs Maximum key length of 40 characters Maximum value length of 500 characters
    #[serde(
        rename = "metadata",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata: Option<Option<::std::collections::HashMap<String, String>>>,
    /// Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank.
    #[serde(
        rename = "origination_account_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub origination_account_id: Option<Option<String>>,
    /// The currency of the transfer amount. The default value is \"USD\".
    #[serde(rename = "iso_currency_code", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
}

impl TransferCreateRequest {
    /// Defines the request schema for `/transfer/create`
    pub fn new(authorization_id: String, description: String) -> TransferCreateRequest {
        TransferCreateRequest {
            client_id: None,
            secret: None,
            idempotency_key: None,
            access_token: None,
            account_id: None,
            payment_profile_token: None,
            authorization_id,
            r#type: None,
            network: None,
            amount: None,
            description,
            ach_class: None,
            user: None,
            metadata: None,
            origination_account_id: None,
            iso_currency_code: None,
        }
    }
}
