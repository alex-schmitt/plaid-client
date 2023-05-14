/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsGetRequest : InstitutionsGetRequest defines the request schema for `/institutions/get`

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct InstitutionsGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The total number of Institutions to return.
    #[serde(rename = "count")]
    pub count: i32,
    /// The number of Institutions to skip.
    #[serde(rename = "offset")]
    pub offset: i32,
    /// Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard.  In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
    #[serde(rename = "country_codes")]
    pub country_codes: Vec<crate::models::CountryCode>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::InstitutionsGetRequestOptions>>,
}

impl InstitutionsGetRequest {
    /// InstitutionsGetRequest defines the request schema for `/institutions/get`
    pub fn new(
        count: i32,
        offset: i32,
        country_codes: Vec<crate::models::CountryCode>,
    ) -> InstitutionsGetRequest {
        InstitutionsGetRequest {
            client_id: None,
            secret: None,
            count,
            offset,
            country_codes,
            options: None,
        }
    }
}
