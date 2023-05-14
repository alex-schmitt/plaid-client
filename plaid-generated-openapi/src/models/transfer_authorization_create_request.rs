/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationCreateRequest : Defines the request schema for `/transfer/authorization/create`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct TransferAuthorizationCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The Plaid `access_token` for the account that will be debited or credited. Required if not using `payment_profile_token`.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// The Plaid `account_id` corresponding to the end-user account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited. Defaults to the account configured during onboarding.
    #[serde(
        rename = "funding_account_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_account_id: Option<Option<String>>,
    /// The payment profile token associated with the Payment Profile that will be debited or credited. Required if not using `access_token`.
    #[serde(
        rename = "payment_profile_token",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_profile_token: Option<String>,
    #[serde(rename = "type")]
    pub r#type: crate::models::TransferType,
    #[serde(rename = "network")]
    pub network: crate::models::TransferNetwork,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "ach_class", skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<crate::models::AchClass>,
    #[serde(rename = "user")]
    pub user: crate::models::TransferAuthorizationUserInRequest,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<crate::models::TransferAuthorizationDevice>,
    /// Plaid's unique identifier for the origination account for this authorization. If not specified, the default account will be used.
    #[serde(
        rename = "origination_account_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub origination_account_id: Option<String>,
    /// The currency of the transfer amount. The default value is \"USD\".
    #[serde(rename = "iso_currency_code", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /// A random key provided by the client, per unique authorization. Maximum of 50 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create an authorization fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single authorization is created.  Failure to provide this key may result in duplicate charges.  Required for guaranteed ACH customers.
    #[serde(
        rename = "idempotency_key",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub idempotency_key: Option<Option<String>>,
    /// Required for Guarantee. If the end user is initiating the specific transfer themselves via an interactive UI, this should be `true`; for automatic recurring payments where the end user is not actually initiating each individual transfer, it should be `false`.
    #[serde(
        rename = "user_present",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_present: Option<Option<bool>>,
    /// If set to `false`, Plaid will not offer a `guarantee_decision` for this request(Guarantee customers only).
    #[serde(
        rename = "with_guarantee",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub with_guarantee: Option<Option<bool>>,
    /// The unique identifier returned by Plaid's [beacon](https://plaid.com/docs/transfer/guarantee/#using-a-beacon) when it is run on your webpage. Required for Guarantee customers who are not using [Transfer UI](https://plaid.com/docs/transfer/using-transfer-ui/) and have a web checkout experience.
    #[serde(
        rename = "beacon_session_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub beacon_session_id: Option<Option<String>>,
    /// The Plaid client ID that is the originator of this transfer. Only needed if creating transfers on behalf of another client as a third-party sender (TPS).
    #[serde(
        rename = "originator_client_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub originator_client_id: Option<Option<String>>,
}

impl TransferAuthorizationCreateRequest {
    /// Defines the request schema for `/transfer/authorization/create`
    pub fn new(
        r#type: crate::models::TransferType,
        network: crate::models::TransferNetwork,
        amount: String,
        user: crate::models::TransferAuthorizationUserInRequest,
    ) -> TransferAuthorizationCreateRequest {
        TransferAuthorizationCreateRequest {
            client_id: None,
            secret: None,
            access_token: None,
            account_id: None,
            funding_account_id: None,
            payment_profile_token: None,
            r#type,
            network,
            amount,
            ach_class: None,
            user,
            device: None,
            origination_account_id: None,
            iso_currency_code: None,
            idempotency_key: None,
            user_present: None,
            with_guarantee: None,
            beacon_session_id: None,
            originator_client_id: None,
        }
    }
}