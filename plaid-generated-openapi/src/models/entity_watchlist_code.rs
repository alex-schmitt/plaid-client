/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// EntityWatchlistCode : Shorthand identifier for a specific screening list for entities.

/// Shorthand identifier for a specific screening list for entities.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
pub enum EntityWatchlistCode {
    #[serde(rename = "CA_CON")]
    CaCon,
    #[serde(rename = "EU_CON")]
    EuCon,
    #[serde(rename = "IZ_SOE")]
    IzSoe,
    #[serde(rename = "IZ_UNC")]
    IzUnc,
    #[serde(rename = "IZ_WBK")]
    IzWbk,
    #[serde(rename = "US_CAP")]
    UsCap,
    #[serde(rename = "US_FSE")]
    UsFse,
    #[serde(rename = "US_MBS")]
    UsMbs,
    #[serde(rename = "US_SDN")]
    UsSdn,
    #[serde(rename = "US_SSI")]
    UsSsi,
    #[serde(rename = "US_CMC")]
    UsCmc,
    #[serde(rename = "US_UVL")]
    UsUvl,
    #[serde(rename = "AU_CON")]
    AuCon,
    #[serde(rename = "UK_HMC")]
    UkHmc,
}

impl ToString for EntityWatchlistCode {
    fn to_string(&self) -> String {
        match self {
            Self::CaCon => String::from("CA_CON"),
            Self::EuCon => String::from("EU_CON"),
            Self::IzSoe => String::from("IZ_SOE"),
            Self::IzUnc => String::from("IZ_UNC"),
            Self::IzWbk => String::from("IZ_WBK"),
            Self::UsCap => String::from("US_CAP"),
            Self::UsFse => String::from("US_FSE"),
            Self::UsMbs => String::from("US_MBS"),
            Self::UsSdn => String::from("US_SDN"),
            Self::UsSsi => String::from("US_SSI"),
            Self::UsCmc => String::from("US_CMC"),
            Self::UsUvl => String::from("US_UVL"),
            Self::AuCon => String::from("AU_CON"),
            Self::UkHmc => String::from("UK_HMC"),
        }
    }
}

impl Default for EntityWatchlistCode {
    fn default() -> EntityWatchlistCode {
        Self::CaCon
    }
}
