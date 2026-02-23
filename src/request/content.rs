//! Content object — Section 3.2.16
//!
//! This object describes the content in which the impression will appear,
//! which may be syndicated or non-syndicated content. This object may be
//! useful when syndicated content contains impressions and does not
//! necessarily match the publisher's general content. The exchange might or
//! might not have knowledge of the page where the content is running, because
//! of the syndication method.

use serde::{Deserialize, Serialize};

use super::{channel::Channel, data::Data, network::Network, producer::Producer};

/// Content in which the impression will appear — Section 3.2.16
///
/// # Example
/// ```rust
/// use openrtb26::Content;
///
/// let content = Content {
///     id: Some("1234567".to_string()),
///     series: Some("All About Cars".to_string()),
///     season: Some("2".to_string()),
///     episode: Some(23),
///     title: Some("Car Show".to_string()),
///     cat: Some(vec!["IAB2-2".to_string()]),
///     keywords: Some("keyword-a,keyword-b,keyword-c".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Content {
    /// Publisher-provided ID uniquely identifying the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Episode number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i32>,

    /// Content title.
    ///
    /// *Video examples:* `"Search Committee"` (television),
    /// `"A New Hope"` (movie), or `"Endgame"` (made for web).
    ///
    /// *Non-video example:* `"Why an Antarctic Glacier Is Melting So Quickly"`
    /// (Time magazine article).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Content series.
    ///
    /// *Video examples:* `"The Office"` (television), `"Star Wars"` (movie),
    /// or `"Arby 'N' The Chief"` (made for web).
    ///
    /// *Non-video example:* `"Ecocentric"` (Time Magazine blog).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    /// Content season, e.g. `"Season 3"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,

    /// Artist credited with the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    /// Genre that best describes the content (e.g., `"rock"`, `"pop"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    /// The taxonomy in use for `genres`.
    ///
    /// Refer to AdCOM 1.0 List: Category Taxonomies.
    /// If no `gtax` field is supplied, Content Category Taxonomy 3.1 is
    /// assumed. Default `9`.
    #[serde(default = "default_gtax", skip_serializing_if = "is_default_gtax")]
    pub gtax: i32,

    /// Unique ID(s) for the genre of the content as listed in the taxonomy
    /// defined by the `gtax` field.
    ///
    /// If no `gtax` field is supplied, a subset of the rows listed in the
    /// CTV Genre Mapping of Content Category Taxonomy 3.1 are assumed.
    ///
    /// See Section 7.13 of Implementation Guidance for additional detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,

    /// Album to which the content belongs; typically for audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    /// International Standard Recording Code conforming to ISO-3901.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,

    /// Details about the content [`Producer`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<Producer>,

    /// URL of the content, for buy-side contextualisation or review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The taxonomy in use for `cat`.
    ///
    /// Refer to AdCOM 1.0 List: Category Taxonomies.
    /// Default `1` = IAB Content Category Taxonomy 1.0.
    #[serde(default = "default_cattax", skip_serializing_if = "is_default_cattax")]
    pub cattax: i32,

    /// Array of IAB Tech Lab content categories that describe the content.
    /// The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Production quality.
    ///
    /// Refer to AdCOM 1.0 List: Production Qualities.
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | 1 | Professionally Produced |
    /// | 2 | Prosumer |
    /// | 3 | User Generated (UGC) |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prodq: Option<i32>,

    /// Type of content (game, video, text, etc.).
    ///
    /// Refer to AdCOM 1.0 List: Content Contexts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i32>,

    /// Content rating (e.g., MPAA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentrating: Option<String>,

    /// User rating of the content (e.g., number of stars, likes, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userrating: Option<String>,

    /// Media rating per IQG guidelines.
    ///
    /// Refer to AdCOM 1.0 List: Media Ratings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<i32>,

    /// Comma-separated list of keywords describing the content.
    /// Only one of `keywords` or `kwarray` may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Array of keywords about the content.
    /// Only one of `keywords` or `kwarray` may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwarray: Option<Vec<String>>,

    /// `0` = not live, `1` = content is live (e.g., stream, live blog).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livestream: Option<i32>,

    /// `0` = indirect, `1` = direct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourcerelationship: Option<i32>,

    /// Length of content in seconds; appropriate for video or audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Content language using ISO-639-1-alpha-2.
    /// Only one of `language` or `langb` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Content language using IETF BCP 47.
    /// Only one of `language` or `langb` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langb: Option<String>,

    /// Indicator of whether the content is embeddable (e.g., an embeddable
    /// video player). `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddable: Option<i32>,

    /// Additional content data. Each [`Data`] object represents a different
    /// data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Data>>,

    /// Details about the [`Network`] the content is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,

    /// Details about the [`Channel`] the content is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── serde helpers ─────────────────────────────────────────────────────────────

fn default_cattax() -> i32 {
    1
}

fn is_default_cattax(v: &i32) -> bool {
    *v == 1
}

fn default_gtax() -> i32 {
    9
}

fn is_default_gtax(v: &i32) -> bool {
    *v == 9
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_empty_roundtrip() {
        let c = Content::default();
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(json, "{}");
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }

    #[test]
    fn content_minimal_roundtrip() {
        let c = Content {
            id: Some("1234567".to_string()),
            title: Some("Car Show".to_string()),
            series: Some("All About Cars".to_string()),
            season: Some("2".to_string()),
            episode: Some(23),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"id\":\"1234567\""));
        assert!(json.contains("\"title\":\"Car Show\""));
        assert!(json.contains("\"series\":\"All About Cars\""));
        assert!(json.contains("\"season\":\"2\""));
        assert!(json.contains("\"episode\":23"));
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }

    #[test]
    fn content_default_cattax_not_serialised() {
        let c = Content {
            cat: Some(vec!["IAB2-2".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(!json.contains("\"cattax\""));
    }

    #[test]
    fn content_non_default_cattax_serialised() {
        let c = Content {
            cattax: 2,
            cat: Some(vec!["IAB2-2".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"cattax\":2"));
    }

    #[test]
    fn content_default_gtax_not_serialised() {
        let c = Content {
            genres: Some(vec!["1".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(!json.contains("\"gtax\""));
    }

    #[test]
    fn content_non_default_gtax_serialised() {
        let c = Content {
            gtax: 5,
            genres: Some(vec!["100".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"gtax\":5"));
    }

    #[test]
    fn content_with_producer_roundtrip() {
        let c = Content {
            id: Some("c-001".to_string()),
            producer: Some(Producer {
                id: Some("producer-001".to_string()),
                name: Some("Warner Bros".to_string()),
                domain: Some("warnerbros.com".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"producer\""));
        assert!(json.contains("\"Warner Bros\""));
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }

    #[test]
    fn content_with_network_and_channel_roundtrip() {
        let c = Content {
            id: Some("c-002".to_string()),
            network: Some(Network {
                id: Some("net-001".to_string()),
                name: Some("ABC".to_string()),
                domain: Some("abc.com".to_string()),
                ext: None,
            }),
            channel: Some(Channel {
                id: Some("ch-001".to_string()),
                name: Some("WABC-TV".to_string()),
                domain: Some("abc7ny.com".to_string()),
                ext: None,
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"network\""));
        assert!(json.contains("\"channel\""));
        assert!(json.contains("\"ABC\""));
        assert!(json.contains("\"WABC-TV\""));
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }

    #[test]
    fn content_with_data_roundtrip() {
        use super::super::segment::{Data, Segment};
        let c = Content {
            data: Some(vec![Data {
                id: Some("dp-1".to_string()),
                name: Some("DataProvider".to_string()),
                segment: Some(vec![Segment {
                    id: Some("seg-1".to_string()),
                    name: Some("sports fans".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"data\""));
        assert!(json.contains("\"segment\""));
        assert!(json.contains("\"sports fans\""));
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }

    #[test]
    fn content_language_and_langb_roundtrip() {
        let c = Content {
            language: Some("en".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"language\":\"en\""));
        assert!(!json.contains("\"langb\""));
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }

    #[test]
    fn content_livestream_embeddable_roundtrip() {
        let c = Content {
            livestream: Some(1),
            embeddable: Some(0),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"livestream\":1"));
        assert!(json.contains("\"embeddable\":0"));
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }

    #[test]
    fn content_kwarray_roundtrip() {
        let c = Content {
            kwarray: Some(vec!["cars".to_string(), "racing".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"kwarray\":[\"cars\",\"racing\"]"));
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }

    /// Replicate the content object from spec Example 4 (Video).
    #[test]
    fn spec_example_4_content_from_json() {
        let raw = r#"{
            "id": "1234567",
            "series": "All About Cars",
            "season": "2",
            "episode": 23,
            "title": "Car Show",
            "cat": ["IAB2-2"],
            "keywords": "keyword-a,keyword-b,keyword-c"
        }"#;
        let c: Content = serde_json::from_str(raw).unwrap();
        assert_eq!(c.id.as_deref(), Some("1234567"));
        assert_eq!(c.series.as_deref(), Some("All About Cars"));
        assert_eq!(c.season.as_deref(), Some("2"));
        assert_eq!(c.episode, Some(23));
        assert_eq!(c.title.as_deref(), Some("Car Show"));
        let cats = c.cat.as_ref().unwrap();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0], "IAB2-2");
        assert_eq!(c.keywords.as_deref(), Some("keyword-a,keyword-b,keyword-c"));
    }

    #[test]
    fn content_full_roundtrip() {
        let c = Content {
            id: Some("c-full".to_string()),
            episode: Some(5),
            title: Some("Episode Title".to_string()),
            series: Some("My Series".to_string()),
            season: Some("Season 1".to_string()),
            artist: Some("Artist Name".to_string()),
            genre: Some("drama".to_string()),
            gtax: 9,
            genres: Some(vec!["100".to_string(), "200".to_string()]),
            album: Some("My Album".to_string()),
            isrc: Some("USUM71703692".to_string()),
            producer: Some(Producer {
                id: Some("prod-1".to_string()),
                name: Some("Production Co.".to_string()),
                ..Default::default()
            }),
            url: Some("https://example.com/content/c-full".to_string()),
            cattax: 1,
            cat: Some(vec!["IAB1-1".to_string()]),
            prodq: Some(1),
            context: Some(1),
            contentrating: Some("PG-13".to_string()),
            userrating: Some("4.5".to_string()),
            qagmediarating: Some(1),
            keywords: Some("drama,series".to_string()),
            kwarray: None,
            livestream: Some(0),
            sourcerelationship: Some(1),
            len: Some(1800),
            language: Some("en".to_string()),
            langb: None,
            embeddable: Some(1),
            data: None,
            network: Some(Network {
                name: Some("HBO".to_string()),
                ..Default::default()
            }),
            channel: Some(Channel {
                name: Some("HBO Max".to_string()),
                ..Default::default()
            }),
            ext: None,
        };
        let json = serde_json::to_string(&c).unwrap();
        let decoded: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(c, decoded);
    }
}
