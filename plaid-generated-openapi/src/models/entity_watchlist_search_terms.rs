/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// EntityWatchlistSearchTerms : Search inputs for creating an entity watchlist screening

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct EntityWatchlistSearchTerms {
    /// ID of the associated entity program.
    #[serde(rename = "entity_watchlist_program_id")]
    pub entity_watchlist_program_id: String,
    /// The name of the organization being screened.
    #[serde(rename = "legal_name")]
    pub legal_name: String,
    /// The numeric or alphanumeric identifier associated with this document.
    #[serde(
        rename = "document_number",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_number: Option<Option<String>>,
    /// A valid email address.
    #[serde(
        rename = "email_address",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub email_address: Option<Option<String>>,
    /// Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(
        rename = "country",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub country: Option<Option<String>>,
    /// A phone number in E.164 format.
    #[serde(
        rename = "phone_number",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number: Option<Option<String>>,
    /// An 'http' or 'https' URL (must begin with either of those).
    #[serde(
        rename = "url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<Option<String>>,
}

impl EntityWatchlistSearchTerms {
    /// Search inputs for creating an entity watchlist screening
    pub fn new(
        entity_watchlist_program_id: String,
        legal_name: String,
    ) -> EntityWatchlistSearchTerms {
        EntityWatchlistSearchTerms {
            entity_watchlist_program_id,
            legal_name,
            document_number: None,
            email_address: None,
            country: None,
            phone_number: None,
            url: None,
        }
    }
}