/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IndividualWatchlistCode : Shorthand identifier for a specific screening list for individuals.

/// Shorthand identifier for a specific screening list for individuals.
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
pub enum IndividualWatchlistCode {
    #[serde(rename = "AU_CON")]
    AuCon,
    #[serde(rename = "CA_CON")]
    CaCon,
    #[serde(rename = "EU_CON")]
    EuCon,
    #[serde(rename = "IZ_CIA")]
    IzCia,
    #[serde(rename = "IZ_IPL")]
    IzIpl,
    #[serde(rename = "IZ_PEP")]
    IzPep,
    #[serde(rename = "IZ_UNC")]
    IzUnc,
    #[serde(rename = "IZ_WBK")]
    IzWbk,
    #[serde(rename = "UK_HMC")]
    UkHmc,
    #[serde(rename = "US_DPL")]
    UsDpl,
    #[serde(rename = "US_DTC")]
    UsDtc,
    #[serde(rename = "US_FBI")]
    UsFbi,
    #[serde(rename = "US_FSE")]
    UsFse,
    #[serde(rename = "US_ISN")]
    UsIsn,
    #[serde(rename = "US_MBS")]
    UsMbs,
    #[serde(rename = "US_PLC")]
    UsPlc,
    #[serde(rename = "US_SDN")]
    UsSdn,
    #[serde(rename = "US_SSI")]
    UsSsi,
    #[serde(rename = "SG_SOF")]
    SgSof,
    #[serde(rename = "TR_TWL")]
    TrTwl,
    #[serde(rename = "TR_DFD")]
    TrDfd,
    #[serde(rename = "TR_FOR")]
    TrFor,
    #[serde(rename = "TR_WMD")]
    TrWmd,
    #[serde(rename = "TR_CMB")]
    TrCmb,
}

impl ToString for IndividualWatchlistCode {
    fn to_string(&self) -> String {
        match self {
            Self::AuCon => String::from("AU_CON"),
            Self::CaCon => String::from("CA_CON"),
            Self::EuCon => String::from("EU_CON"),
            Self::IzCia => String::from("IZ_CIA"),
            Self::IzIpl => String::from("IZ_IPL"),
            Self::IzPep => String::from("IZ_PEP"),
            Self::IzUnc => String::from("IZ_UNC"),
            Self::IzWbk => String::from("IZ_WBK"),
            Self::UkHmc => String::from("UK_HMC"),
            Self::UsDpl => String::from("US_DPL"),
            Self::UsDtc => String::from("US_DTC"),
            Self::UsFbi => String::from("US_FBI"),
            Self::UsFse => String::from("US_FSE"),
            Self::UsIsn => String::from("US_ISN"),
            Self::UsMbs => String::from("US_MBS"),
            Self::UsPlc => String::from("US_PLC"),
            Self::UsSdn => String::from("US_SDN"),
            Self::UsSsi => String::from("US_SSI"),
            Self::SgSof => String::from("SG_SOF"),
            Self::TrTwl => String::from("TR_TWL"),
            Self::TrDfd => String::from("TR_DFD"),
            Self::TrFor => String::from("TR_FOR"),
            Self::TrWmd => String::from("TR_WMD"),
            Self::TrCmb => String::from("TR_CMB"),
        }
    }
}

impl Default for IndividualWatchlistCode {
    fn default() -> IndividualWatchlistCode {
        Self::AuCon
    }
}
