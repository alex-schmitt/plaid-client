/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RiskCheckLinkedService : An enum indicating the type of a linked service.

/// An enum indicating the type of a linked service.
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
pub enum RiskCheckLinkedService {
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "ebay")]
    Ebay,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "flickr")]
    Flickr,
    #[serde(rename = "foursquare")]
    Foursquare,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "gravatar")]
    Gravatar,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "lastfm")]
    Lastfm,
    #[serde(rename = "linkedin")]
    Linkedin,
    #[serde(rename = "microsoft")]
    Microsoft,
    #[serde(rename = "myspace")]
    Myspace,
    #[serde(rename = "pinterest")]
    Pinterest,
    #[serde(rename = "skype")]
    Skype,
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "tumblr")]
    Tumblr,
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "viber")]
    Viber,
    #[serde(rename = "vimeo")]
    Vimeo,
    #[serde(rename = "weibo")]
    Weibo,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "yahoo")]
    Yahoo,
    #[serde(rename = "airbnb")]
    Airbnb,
    #[serde(rename = "amazon")]
    Amazon,
    #[serde(rename = "booking")]
    Booking,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "kakao")]
    Kakao,
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "qzone")]
    Qzone,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "snapchat")]
    Snapchat,
    #[serde(rename = "zalo")]
    Zalo,
}

impl ToString for RiskCheckLinkedService {
    fn to_string(&self) -> String {
        match self {
            Self::Apple => String::from("apple"),
            Self::Ebay => String::from("ebay"),
            Self::Facebook => String::from("facebook"),
            Self::Flickr => String::from("flickr"),
            Self::Foursquare => String::from("foursquare"),
            Self::Github => String::from("github"),
            Self::Google => String::from("google"),
            Self::Gravatar => String::from("gravatar"),
            Self::Instagram => String::from("instagram"),
            Self::Lastfm => String::from("lastfm"),
            Self::Linkedin => String::from("linkedin"),
            Self::Microsoft => String::from("microsoft"),
            Self::Myspace => String::from("myspace"),
            Self::Pinterest => String::from("pinterest"),
            Self::Skype => String::from("skype"),
            Self::Spotify => String::from("spotify"),
            Self::Telegram => String::from("telegram"),
            Self::Tumblr => String::from("tumblr"),
            Self::Twitter => String::from("twitter"),
            Self::Viber => String::from("viber"),
            Self::Vimeo => String::from("vimeo"),
            Self::Weibo => String::from("weibo"),
            Self::Whatsapp => String::from("whatsapp"),
            Self::Yahoo => String::from("yahoo"),
            Self::Airbnb => String::from("airbnb"),
            Self::Amazon => String::from("amazon"),
            Self::Booking => String::from("booking"),
            Self::Discord => String::from("discord"),
            Self::Kakao => String::from("kakao"),
            Self::Ok => String::from("ok"),
            Self::Qzone => String::from("qzone"),
            Self::Line => String::from("line"),
            Self::Snapchat => String::from("snapchat"),
            Self::Zalo => String::from("zalo"),
        }
    }
}

impl Default for RiskCheckLinkedService {
    fn default() -> RiskCheckLinkedService {
        Self::Apple
    }
}
