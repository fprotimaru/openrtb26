//! Segment, Data, User, Publisher, Producer, Network, Channel, Refresh, RefSettings objects
//!
//! - [`Segment`]     — Section 3.2.22 — A key-value pair of audience data.
//! - [`Data`]        — Section 3.2.21 — A collection of audience data from a single source.
//! - [`User`]        — Section 3.2.20 — Human user of the device; the advertising audience.
//! - [`Publisher`]   — Section 3.2.15 — Entity that controls the content and distributes the site or app.
//! - [`Producer`]    — Section 3.2.17 — Producer of the content; not necessarily the publisher.
//! - [`Network`]     — Section 3.2.23 — Network on which an ad will be displayed.
//! - [`Channel`]     — Section 3.2.24 — Channel on which an ad will be displayed.
//! - [`Refresh`]     — Section 3.2.33 — Details about ad slots being refreshed automatically.
//! - [`RefSettings`] — Section 3.2.34 — Information on how often and what triggers an ad slot refresh.

use serde::{Deserialize, Serialize};

use super::eid::Eid;
use super::geo::Geo;

// ── Segment ───────────────────────────────────────────────────────────────────

/// A single key-value pair of audience data — Section 3.2.22
///
/// Segment objects are essentially key-value pairs that convey specific units
/// of data. The parent [`Data`] object is a collection of such values from a
/// given data provider. The specific segment names and value options must be
/// published by the exchange *a priori* to its bidders.
///
/// # Example
/// ```rust
/// use openrtb26::Segment;
///
/// let seg = Segment {
///     id: Some("12341318394918".to_string()),
///     name: Some("auto intenders".to_string()),
///     value: None,
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    /// ID of the data segment specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the data segment specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// String representation of the data segment value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Data ──────────────────────────────────────────────────────────────────────

/// A collection of audience data from a single source — Section 3.2.21
///
/// The `Data` and `Segment` objects together allow additional data about the
/// related object (e.g., user, content) to be specified. This data may be from
/// multiple sources whether from the exchange itself or third parties as
/// specified by the `id` field. A bid request can mix data objects from
/// multiple providers.
///
/// The specific data providers in use should be published by the exchange a
/// priori to its bidders.
///
/// # Example
/// ```rust
/// use openrtb26::{Data, Segment};
///
/// let data = Data {
///     id: Some("6".to_string()),
///     name: Some("Data Provider 1".to_string()),
///     segment: Some(vec![
///         Segment {
///             id: Some("12341318394918".to_string()),
///             name: Some("auto intenders".to_string()),
///             ..Default::default()
///         },
///     ]),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// Exchange-specific ID for the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Exchange-specific name for the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// An array of Extended Content IDs, representing one or more identifiers
    /// for the video or audio content from the ID source specified in the
    /// `name` field of the `Data` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cids: Option<Vec<String>>,

    /// Array of [`Segment`] objects that contain the actual data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Vec<Segment>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── User ──────────────────────────────────────────────────────────────────────

/// Human user of the device; the advertising audience — Section 3.2.20
///
/// This object contains information known or derived about the human user of
/// the device. The user `id` is an exchange artifact and may be subject to
/// rotation or other privacy policies. However, when present, this user ID
/// should be stable long enough to serve reasonably as the basis for frequency
/// capping and retargeting.
///
/// # Deprecation notice
/// The `yob` (year of birth) and `gender` fields were deprecated as of
/// OpenRTB 2.6. New implementations should not populate them.
///
/// # Example
/// ```rust
/// use openrtb26::User;
///
/// let user = User {
///     id: Some("55816b39711f9b5acf3b90e313ed29e51665623f".to_string()),
///     buyeruid: Some("545678765467876567898765678987654".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Exchange-specific ID for the user.
    ///
    /// Unless prior arrangements have been made between the buyer and the
    /// seller directly, the value in this field is expected to be the
    /// exchange's user ID from its cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Buyer-specific ID for the user as mapped by the exchange for the buyer.
    ///
    /// Unless prior arrangements have been made between the buyer and the
    /// seller directly, the value in this field is expected to be derived from
    /// an ID sync.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyeruid: Option<String>,

    /// **DEPRECATED as of OpenRTB 2.6.** Year of birth as a 4-digit integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yob: Option<i32>,

    /// **DEPRECATED as of OpenRTB 2.6.**
    /// Gender, where `"M"` = male, `"F"` = female, `"O"` = known to be other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// Comma-separated list of keywords, interests, or intent.
    /// Only one of `keywords` or `kwarray` may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Array of keywords about the user.
    /// Only one of `keywords` or `kwarray` may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwarray: Option<Vec<String>>,

    /// Optional feature to pass bidder data that was set in the exchange's
    /// cookie. The string must be in base85 cookie safe characters and be in
    /// any format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,

    /// Location of the user's home base defined by a [`Geo`] object.
    /// This is not necessarily their current location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    /// Additional user data. Each [`Data`] object represents a different data
    /// source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Data>>,

    /// When GDPR regulations are in effect this attribute contains the
    /// Transparency and Consent Framework's Consent String data structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<String>,

    /// Details for support of a standard protocol for multiple third party
    /// identity providers. Array of [`Eid`] objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eids: Option<Vec<Eid>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Publisher ─────────────────────────────────────────────────────────────────

/// Entity that directly supplies inventory to the exchange — Section 3.2.15
///
/// This object describes the entity who directly supplies inventory to and is
/// paid by the exchange. This may be a publisher, intermediary exchange, ad
/// network, etc.
///
/// # Example
/// ```rust
/// use openrtb26::Publisher;
///
/// let publisher = Publisher {
///     id: Some("8953".to_string()),
///     name: Some("foobar.com".to_string()),
///     domain: Some("foobar.com".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Publisher {
    /// Exchange-specific seller ID. Every ID must map to only a single entity
    /// that is paid for inventory transacted via that ID. Corresponds to a
    /// `seller_id` of a seller in the exchange's `sellers.json` file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Seller name (may be aliased at the seller's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The taxonomy in use for `cat`.
    /// Refer to AdCOM 1.0 List: Category Taxonomies.
    /// Default `1` = IAB Content Category Taxonomy 1.0.
    #[serde(default = "default_cattax", skip_serializing_if = "is_default_cattax")]
    pub cattax: i32,

    /// Array of IAB Tech Lab content categories of the publisher.
    /// The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the seller (e.g., `"seller.com"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Producer ──────────────────────────────────────────────────────────────────

/// Producer of the content — Section 3.2.17
///
/// This object defines the producer of the content in which the ad will be
/// shown. This is particularly useful when the content is syndicated and may
/// be distributed through different publishers and thus when the producer and
/// publisher are not necessarily the same entity.
///
/// # Example
/// ```rust
/// use openrtb26::Producer;
///
/// let producer = Producer {
///     id: Some("producer-001".to_string()),
///     name: Some("Warner Bros".to_string()),
///     domain: Some("warnerbros.com".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Producer {
    /// Content producer or originator ID.
    /// Useful if content is syndicated and may be posted on a site using embed
    /// tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Content producer or originator name (e.g., `"Warner Bros"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The taxonomy in use for `cat`.
    /// Refer to AdCOM 1.0 List: Category Taxonomies.
    /// Default `1` = IAB Content Category Taxonomy 1.0.
    #[serde(default = "default_cattax", skip_serializing_if = "is_default_cattax")]
    pub cattax: i32,

    /// Array of IAB Tech Lab content categories that describe the content
    /// producer. The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the content producer (e.g., `"producer.com"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Network ───────────────────────────────────────────────────────────────────

/// Network on which an ad will be displayed — Section 3.2.23
///
/// A Network is defined as the parent entity of the [`Channel`] object's
/// entity for the purposes of organizing Channels. Examples are companies that
/// own and/or license a collection of content channels (Viacom, Discovery,
/// CBS, WarnerMedia, Turner and others), or a studio that creates such content
/// and self-distributes content.
///
/// `name` is a human-readable field while `domain` and `id` can be used for
/// reporting and targeting purposes.
///
/// # Example
/// ```rust
/// use openrtb26::Network;
///
/// let network = Network {
///     id: Some("net-001".to_string()),
///     name: Some("ABC".to_string()),
///     domain: Some("abc.com".to_string()),
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Network {
    /// A unique identifier assigned by the publisher.
    /// This may not be a unique identifier across all supply sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Network the content is on (e.g., a TV network like `"ABC"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The primary domain of the network (e.g. `"abc.com"` in the case of the
    /// network ABC).
    ///
    /// It is recommended to include the top private domain (PSL+1) for DSP
    /// targeting normalisation purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Channel ───────────────────────────────────────────────────────────────────

/// Channel on which an ad will be displayed — Section 3.2.24
///
/// A Channel is defined as the entity that curates a content library, or
/// stream within a brand name for viewers. Examples are specific view
/// selectable "channels" within linear and streaming television (MTV, HGTV,
/// CNN, BBC One, etc.) or a specific stream of audio content commonly called
/// "stations."
///
/// `name` is a human-readable field while `domain` and `id` can be used for
/// reporting and targeting purposes.
///
/// # Example
/// ```rust
/// use openrtb26::Channel;
///
/// let channel = Channel {
///     id: Some("ch-001".to_string()),
///     name: Some("WABC-TV".to_string()),
///     domain: Some("abc7ny.com".to_string()),
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// A unique identifier assigned by the publisher.
    /// This may not be a unique identifier across all supply sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Channel the content is on (e.g., a TV channel like `"WABC-TV"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The primary domain of the channel (e.g., `"abc7ny.com"` in the case of
    /// the local channel WABC-TV).
    ///
    /// It is recommended to include the top private domain (PSL+1) for DSP
    /// targeting normalisation purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── RefSettings ───────────────────────────────────────────────────────────────

/// Trigger type and minimum interval for an ad-slot auto-refresh — Section 3.2.34
///
/// Information on how often and what triggers an ad slot being refreshed.
///
/// # Example
/// ```rust
/// use openrtb26::RefSettings;
///
/// let settings = RefSettings {
///     reftype: 1,
///     minint: Some(30),
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RefSettings {
    /// The type of the declared auto refresh.
    ///
    /// Refer to AdCOM 1.0 List: Auto Refresh Triggers.
    ///
    /// Default `0`.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub reftype: i32,

    /// The minimum refresh interval in seconds. This applies to all refresh
    /// types. This is the (uninterrupted) time the ad creative will be
    /// rendered before refreshing to the next creative.
    ///
    /// If the field is absent, the exposure time is unknown. This field does
    /// not account for viewability or external factors such as a user leaving
    /// a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minint: Option<i32>,

    /// Placeholder for vendor-specific extensions to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Refresh ───────────────────────────────────────────────────────────────────

/// Details about ad slots being refreshed automatically — Section 3.2.33
///
/// When an ad slot is configured to automatically refresh, this object
/// communicates the mechanics of that refresh behaviour to buyers so they can
/// make appropriate bidding decisions.
///
/// # Example
/// ```rust
/// use openrtb26::{Refresh, RefSettings};
///
/// let refresh = Refresh {
///     refsettings: Some(vec![RefSettings {
///         reftype: 1,
///         minint: Some(30),
///         ext: None,
///     }]),
///     count: Some(3),
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Refresh {
    /// A [`RefSettings`] object (or array thereof) describing the mechanics of
    /// how an ad placement automatically refreshes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refsettings: Option<Vec<RefSettings>>,

    /// The number of times this ad slot has been refreshed since the last page
    /// load.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// Placeholder for vendor-specific extensions to this object.
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

fn is_zero_i32(v: &i32) -> bool {
    *v == 0
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Segment ───────────────────────────────────────────────────────────────

    #[test]
    fn segment_empty_roundtrip() {
        let s = Segment::default();
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "{}");
        let decoded: Segment = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn segment_full_roundtrip() {
        let s = Segment {
            id: Some("12341318394918".to_string()),
            name: Some("auto intenders".to_string()),
            value: Some("true".to_string()),
            ext: None,
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"id\":\"12341318394918\""));
        assert!(json.contains("\"name\":\"auto intenders\""));
        assert!(json.contains("\"value\":\"true\""));
        let decoded: Segment = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    // ── Data ──────────────────────────────────────────────────────────────────

    #[test]
    fn data_empty_roundtrip() {
        let d = Data::default();
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(json, "{}");
        let decoded: Data = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn data_with_segments_roundtrip() {
        let d = Data {
            id: Some("6".to_string()),
            name: Some("Data Provider 1".to_string()),
            cids: None,
            segment: Some(vec![
                Segment {
                    id: Some("12341318394918".to_string()),
                    name: Some("auto intenders".to_string()),
                    ..Default::default()
                },
                Segment {
                    id: Some("1234131839491234".to_string()),
                    name: Some("auto enthusiasts".to_string()),
                    ..Default::default()
                },
            ]),
            ext: None,
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"id\":\"6\""));
        assert!(json.contains("\"name\":\"Data Provider 1\""));
        assert!(json.contains("\"segment\""));
        let decoded: Data = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn data_with_cids_roundtrip() {
        let d = Data {
            name: Some("content-id-source".to_string()),
            cids: Some(vec![
                "content-id-001".to_string(),
                "content-id-002".to_string(),
            ]),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"cids\":[\"content-id-001\",\"content-id-002\"]"));
        let decoded: Data = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    /// Replicate the data object from spec Example 2 (Expandable Creative).
    #[test]
    fn spec_example_2_data_from_json() {
        let raw = r#"[
            { "id": "12341318394918", "name": "auto intenders" },
            { "id": "1234131839491234", "name": "auto enthusiasts" },
            { "id": "23423424", "name": "data-provider1-age", "value": "30-40" }
        ]"#;
        // Note: in the spec Example 2 the data array on User uses flat objects
        // without a "segment" key. Let's check Data parses gracefully.
        let items: Vec<Data> = serde_json::from_str(raw).unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].id.as_deref(), Some("12341318394918"));
        assert_eq!(items[2].name.as_deref(), Some("data-provider1-age"));
    }

    // ── User ──────────────────────────────────────────────────────────────────

    #[test]
    fn user_empty_roundtrip() {
        let u = User::default();
        let json = serde_json::to_string(&u).unwrap();
        assert_eq!(json, "{}");
        let decoded: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, decoded);
    }

    #[test]
    fn user_id_buyeruid_roundtrip() {
        let u = User {
            id: Some("55816b39711f9b5acf3b90e313ed29e51665623f".to_string()),
            buyeruid: Some("545678765467876567898765678987654".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"id\":\"55816b39711f9b5acf3b90e313ed29e51665623f\""));
        assert!(json.contains("\"buyeruid\":\"545678765467876567898765678987654\""));
        let decoded: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, decoded);
    }

    #[test]
    fn user_with_geo_roundtrip() {
        let u = User {
            id: Some("user-123".to_string()),
            geo: Some(Geo {
                country: Some("USA".to_string()),
                region: Some("CA".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"geo\""));
        assert!(json.contains("\"country\":\"USA\""));
        let decoded: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, decoded);
    }

    #[test]
    fn user_with_data_roundtrip() {
        let u = User {
            id: Some("user-456".to_string()),
            data: Some(vec![Data {
                id: Some("6".to_string()),
                name: Some("Data Provider 1".to_string()),
                segment: Some(vec![Segment {
                    id: Some("12341318394918".to_string()),
                    name: Some("auto intenders".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"data\""));
        assert!(json.contains("\"segment\""));
        let decoded: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, decoded);
    }

    #[test]
    fn user_with_eids_roundtrip() {
        use super::super::eid::{Eid, Uid};
        let u = User {
            id: Some("user-789".to_string()),
            eids: Some(vec![Eid {
                source: Some("adserver.org".to_string()),
                uids: Some(vec![Uid {
                    id: "111111111111".to_string(),
                    atype: Some(1),
                    ext: None,
                }]),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"eids\""));
        assert!(json.contains("\"adserver.org\""));
        assert!(json.contains("\"111111111111\""));
        let decoded: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, decoded);
    }

    #[test]
    fn user_consent_roundtrip() {
        let u = User {
            consent: Some("CPXxRfAPXxRfAAfKABENB-CgAAAAAAAAAAYgAAAAAAAA".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"consent\""));
        let decoded: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, decoded);
    }

    #[test]
    fn user_kwarray_roundtrip() {
        let u = User {
            kwarray: Some(vec!["tech".to_string(), "gadgets".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"kwarray\":[\"tech\",\"gadgets\"]"));
        let decoded: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, decoded);
    }

    #[test]
    fn user_deprecated_fields_still_parse() {
        let raw = r#"{"id":"u1","yob":1990,"gender":"M"}"#;
        let u: User = serde_json::from_str(raw).unwrap();
        assert_eq!(u.yob, Some(1990));
        assert_eq!(u.gender.as_deref(), Some("M"));
    }
}
