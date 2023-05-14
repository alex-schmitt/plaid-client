/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportFreddie : An object representing an Asset Report with Freddie Mac schema.

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct AssetReportFreddie {
    #[serde(rename = "LOANS")]
    pub loans: crate::models::Loans,
    #[serde(rename = "PARTIES")]
    pub parties: crate::models::Parties,
    #[serde(rename = "SERVICES")]
    pub services: crate::models::Services,
}

impl AssetReportFreddie {
    /// An object representing an Asset Report with Freddie Mac schema.
    pub fn new(
        loans: crate::models::Loans,
        parties: crate::models::Parties,
        services: crate::models::Services,
    ) -> AssetReportFreddie {
        AssetReportFreddie {
            loans,
            parties,
            services,
        }
    }
}
