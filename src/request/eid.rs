//! EID and UID objects — Sections 3.2.27 and 3.2.28
//!
//! Extended identifiers support in the OpenRTB specification allows buyers to
//! use audience data in real-time bidding. An [`Eid`] object can contain one
//! or more [`Uid`] objects from a single source or technology provider.
//!
//! The exchange should ensure that business agreements allow for the sending
//! of this data. See Section 7.12 of Implementation Guidance for additional
//! notes regarding the use of these fields.

use serde::{Deserialize, Serialize};

// ── Uid ───────────────────────────────────────────────────────────────────────

/// A single user identifier from an extended-identifier source — Section 3.2.28
///
/// This object contains a single user identifier provided as part of extended
/// identifiers. The exchange should ensure that business agreements allow for
/// the sending of this data.
///
/// # Example
/// ```rust
/// use openrtb26::Uid;
///
/// let uid = Uid {
///     id: "user-abc-123".to_string(),
///     atype: Some(1),
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Uid {
    /// The identifier for the user.
    pub id: String,

    /// Type of user agent the ID is from. It is highly recommended to set
    /// this, as many DSPs separate app-native IDs from browser-based IDs and
    /// require a type value for ID resolution.
    ///
    /// Refer to AdCOM 1.0 List: Agent Types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atype: Option<i32>,

    /// Placeholder for vendor-specific extensions to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Eid ───────────────────────────────────────────────────────────────────────

/// Extended identifier — Section 3.2.27
///
/// An `Eid` object can contain one or more [`Uid`] objects from a single
/// source or technology provider. The exchange should ensure that business
/// agreements allow for the sending of this data.
///
/// # Example
/// ```rust
/// use openrtb26::{Eid, Uid};
///
/// let eid = Eid {
///     source: Some("adserver.org".to_string()),
///     uids: Some(vec![
///         Uid {
///             id: "TTD-USER-ID-VALUE".to_string(),
///             atype: Some(1),
///             ext: None,
///         },
///     ]),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Eid {
    /// The canonical domain name of the entity (publisher, publisher
    /// monetization company, SSP, Exchange, Header Wrapper, etc.) that caused
    /// the ID array element to be added. This may be the operational domain of
    /// the system if that differs from the parent corporate domain, to
    /// facilitate WHOIS and reverse IP lookups to establish clear ownership.
    ///
    /// This should be the same value as used to identify sellers in an
    /// `ads.txt` file if one exists. For ad tech intermediaries this would be
    /// the domain as used in `ads.txt`. For publishers this would match the
    /// domain in the `site` or `app` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inserter: Option<String>,

    /// Canonical domain of the ID source / provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Technology providing the match method as defined in `mm`.
    ///
    /// In some cases this may be the same value as `inserter`.
    /// When blank it is assumed that the `matcher` equals the `source`.
    /// May be omitted when `mm` is `0`, `1`, or `2`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,

    /// Match method used by the `matcher`.
    ///
    /// Refer to AdCOM 1.0 List: ID Match Methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mm: Option<i32>,

    /// Array of extended ID [`Uid`] objects from the given source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<Vec<Uid>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Uid ───────────────────────────────────────────────────────────────────

    #[test]
    fn uid_minimal_roundtrip() {
        let uid = Uid {
            id: "user-id-abc".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&uid).unwrap();
        assert!(json.contains("\"id\":\"user-id-abc\""));
        assert!(!json.contains("\"atype\""));
        assert!(!json.contains("\"ext\""));
        let decoded: Uid = serde_json::from_str(&json).unwrap();
        assert_eq!(uid, decoded);
    }

    #[test]
    fn uid_with_atype_roundtrip() {
        let uid = Uid {
            id: "device-id-xyz".to_string(),
            atype: Some(2),
            ext: None,
        };
        let json = serde_json::to_string(&uid).unwrap();
        assert!(json.contains("\"id\":\"device-id-xyz\""));
        assert!(json.contains("\"atype\":2"));
        let decoded: Uid = serde_json::from_str(&json).unwrap();
        assert_eq!(uid, decoded);
    }

    #[test]
    fn uid_with_ext_roundtrip() {
        let uid = Uid {
            id: "user-abc".to_string(),
            atype: Some(1),
            ext: Some(serde_json::json!({ "ttd_hash": "deadbeef" })),
        };
        let json = serde_json::to_string(&uid).unwrap();
        assert!(json.contains("\"ext\""));
        assert!(json.contains("\"ttd_hash\""));
        let decoded: Uid = serde_json::from_str(&json).unwrap();
        assert_eq!(uid, decoded);
    }

    // ── Eid ───────────────────────────────────────────────────────────────────

    #[test]
    fn eid_empty_roundtrip() {
        let eid = Eid::default();
        let json = serde_json::to_string(&eid).unwrap();
        assert_eq!(json, "{}");
        let decoded: Eid = serde_json::from_str(&json).unwrap();
        assert_eq!(eid, decoded);
    }

    #[test]
    fn eid_source_and_uids_roundtrip() {
        let eid = Eid {
            source: Some("adserver.org".to_string()),
            uids: Some(vec![
                Uid {
                    id: "TTD-USER-ID-VALUE".to_string(),
                    atype: Some(1),
                    ext: None,
                },
                Uid {
                    id: "TTD-DEVICE-ID".to_string(),
                    atype: Some(2),
                    ext: None,
                },
            ]),
            ..Default::default()
        };
        let json = serde_json::to_string(&eid).unwrap();
        assert!(json.contains("\"source\":\"adserver.org\""));
        assert!(json.contains("\"TTD-USER-ID-VALUE\""));
        assert!(json.contains("\"TTD-DEVICE-ID\""));
        let decoded: Eid = serde_json::from_str(&json).unwrap();
        assert_eq!(eid, decoded);
    }

    #[test]
    fn eid_full_roundtrip() {
        let eid = Eid {
            inserter: Some("publisher.com".to_string()),
            source: Some("liveramp.com".to_string()),
            matcher: Some("liveramp.com".to_string()),
            mm: Some(3),
            uids: Some(vec![Uid {
                id: "RampID-abc123".to_string(),
                atype: Some(3),
                ext: None,
            }]),
            ext: Some(serde_json::json!({ "privacy": "ok" })),
        };
        let json = serde_json::to_string(&eid).unwrap();
        assert!(json.contains("\"inserter\":\"publisher.com\""));
        assert!(json.contains("\"source\":\"liveramp.com\""));
        assert!(json.contains("\"matcher\":\"liveramp.com\""));
        assert!(json.contains("\"mm\":3"));
        assert!(json.contains("\"RampID-abc123\""));
        assert!(json.contains("\"privacy\":\"ok\""));
        let decoded: Eid = serde_json::from_str(&json).unwrap();
        assert_eq!(eid, decoded);
    }

    #[test]
    fn eid_optional_fields_absent_when_not_set() {
        let eid = Eid {
            source: Some("example.com".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&eid).unwrap();
        assert!(!json.contains("\"inserter\""));
        assert!(!json.contains("\"matcher\""));
        assert!(!json.contains("\"mm\""));
        assert!(!json.contains("\"uids\""));
        assert!(!json.contains("\"ext\""));
    }

    #[test]
    fn eid_vec_of_eids_roundtrip() {
        // Tests that a Vec<Eid> (as used in User.eids) round-trips correctly.
        let eids = vec![
            Eid {
                source: Some("ttdid.com".to_string()),
                uids: Some(vec![Uid {
                    id: "uid-ttd-1".to_string(),
                    atype: Some(1),
                    ext: None,
                }]),
                ..Default::default()
            },
            Eid {
                source: Some("liveramp.com".to_string()),
                uids: Some(vec![Uid {
                    id: "uid-lr-1".to_string(),
                    atype: Some(3),
                    ext: None,
                }]),
                ..Default::default()
            },
        ];
        let json = serde_json::to_string(&eids).unwrap();
        let decoded: Vec<Eid> = serde_json::from_str(&json).unwrap();
        assert_eq!(eids, decoded);
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].source.as_deref(), Some("ttdid.com"));
        assert_eq!(decoded[1].source.as_deref(), Some("liveramp.com"));
    }

    /// Demonstrates that `uids` with multiple atype values can be parsed
    /// correctly from a real-world style JSON payload.
    #[test]
    fn eid_parse_from_raw_json() {
        let raw = r#"{
            "source": "adserver.org",
            "uids": [
                { "id": "111111111111", "atype": 1 },
                { "id": "222222222222", "atype": 3 }
            ]
        }"#;
        let eid: Eid = serde_json::from_str(raw).unwrap();
        assert_eq!(eid.source.as_deref(), Some("adserver.org"));
        let uids = eid.uids.as_ref().unwrap();
        assert_eq!(uids.len(), 2);
        assert_eq!(uids[0].id, "111111111111");
        assert_eq!(uids[0].atype, Some(1));
        assert_eq!(uids[1].id, "222222222222");
        assert_eq!(uids[1].atype, Some(3));
    }
}
