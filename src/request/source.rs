//! Source, Regs, DurFloors, and Qty objects
//!
//! - [`Source`]    — Section 3.2.2  — Nature and behavior of the upstream bid-request source.
//! - [`Regs`]      — Section 3.2.3  — Legal, governmental, or industry regulations.
//! - [`DurFloors`] — Section 3.2.35 — Price floors keyed by creative duration range.
//! - [`Qty`]       — Section 3.2.31 — Impression-quantity multiplier (DOOH / CTV).

use serde::{Deserialize, Serialize};

use super::supply_chain::SupplyChain;

// ── Source ────────────────────────────────────────────────────────────────────

/// Nature and behavior of the upstream bid-request source — Section 3.2.2
///
/// The primary purpose of this object is to define post-auction or upstream
/// decisioning when the exchange itself does not control the final decision.
/// A common example is header bidding, but it also applies to upstream server
/// entities such as another RTB exchange, a mediation platform, or an ad
/// server that combines direct campaigns with third-party demand.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// Entity responsible for the final impression sale decision.
    /// `0` = exchange, `1` = upstream source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fd: Option<i32>,

    /// Transaction ID that must be common across all participants in this bid
    /// request (e.g., potentially multiple exchanges).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,

    /// Payment ID chain string containing embedded syntax described in the
    /// TAG Payment ID Protocol v1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pchain: Option<String>,

    /// Supply chain object representing both the links in the supply chain and
    /// an indicator of whether the supply chain is complete.
    /// Refer to Section 3.2.25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schain: Option<SupplyChain>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Regs ──────────────────────────────────────────────────────────────────────

/// Legal, governmental, or industry regulations — Section 3.2.3
///
/// This object contains any legal, governmental, or industry regulations that
/// the sender deems applicable to the request. See Section 7.5 for more
/// details on the flags supporting COPPA, GDPR, and others.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Regs {
    /// Flag indicating if this request is subject to the COPPA regulations
    /// established by the USA FTC.
    /// `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coppa: Option<i32>,

    /// Flag indicating whether or not the request is subject to GDPR
    /// regulations.
    /// `0` = no, `1` = yes. Omission indicates unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdpr: Option<i32>,

    /// Communicates signals regarding consumer privacy under US privacy
    /// regulation. See the US Privacy String specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_privacy: Option<String>,

    /// Contains the Global Privacy Platform's consent string.
    /// See the Global Privacy Platform specification for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpp: Option<String>,

    /// Array of the section(s) of the GPP string which should be applied for
    /// this transaction. GPP Sections 3 (Header) and 4 (Signal Integrity) do
    /// not need to be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpp_sid: Option<Vec<i32>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── DurFloors ─────────────────────────────────────────────────────────────────

/// Price floor for a specific creative-duration range — Section 3.2.35
///
/// This object allows sellers to specify price floors for video and audio
/// creatives whose price varies based on duration. For example:
///
/// - 1–15 seconds → floor of $5.00 CPM
/// - 16–30 seconds → floor of $10.00 CPM
/// - 31+ seconds → floor of $20.00 CPM
///
/// At least one of `mindur` or `maxdur` must be present. If `mindur` is
/// absent the low end is unbounded; if `maxdur` is absent the high end is
/// unbounded. Where multiple ranges overlap, it is up to buyer and seller to
/// coordinate which floor is applicable.
///
/// See Section 7.11 of the Implementation Guidance for additional notes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DurFloors {
    /// Low end of the duration range in seconds (inclusive).
    /// If absent, the low end is unbounded.
    /// Either `mindur` or `maxdur` is required, but not necessarily both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mindur: Option<i32>,

    /// High end of the duration range in seconds (inclusive).
    /// If absent, the high end is unbounded.
    /// Either `mindur` or `maxdur` is required, but not necessarily both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdur: Option<i32>,

    /// Minimum bid for a creative in this duration range, expressed in CPM.
    ///
    /// For creatives whose durations fall outside all defined ranges, the
    /// `bidfloor` at the [`Imp`](super::imp::Imp) level serves as the default
    /// floor. Default `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,

    /// Placeholder for vendor-specific extensions to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Qty ───────────────────────────────────────────────────────────────────────

/// Impression-quantity multiplier — Section 3.2.31
///
/// A programmatic impression is often referred to as a "spot" in Digital
/// Out-Of-Home (DOOH) and CTV, with an impression being a unique member of
/// the audience viewing it. This object provides a standard means of passing
/// a multiplier in the bid request representing the total quantity of
/// billable impressions for adverts that display to more than one person
/// simultaneously.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Qty {
    /// The quantity of billable events deemed to have occurred if this item is
    /// purchased. For example, a DOOH opportunity may be considered to be
    /// 14.2 impressions. Equivalent to `qtyflt` in OpenRTB 3.0.
    ///
    /// **Required.**
    pub multiplier: f64,

    /// Source type of the quantity measurement (e.g., publisher).
    /// Refer to AdCOM 1.0 List: DOOH Multiplier Measurement Source Types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourcetype: Option<i32>,

    /// Top-level business domain name of the measurement vendor providing the
    /// quantity measurement.
    /// Required if `sourcetype` is present and equals `1` (third-party
    /// measurement vendor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Placeholder for vendor-specific extensions to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Source ────────────────────────────────────────────────────────────────

    #[test]
    fn source_empty_roundtrip() {
        let s = Source::default();
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "{}");
        let decoded: Source = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn source_full_roundtrip() {
        let s = Source {
            fd: Some(1),
            tid: Some("abc-txn-123".to_string()),
            pchain: Some("payment-chain-string".to_string()),
            schain: Some(SupplyChain {
                complete: 1,
                nodes: vec![],
                ver: "1.0".to_string(),
                ext: None,
            }),
            ext: Some(serde_json::json!({ "custom": 42 })),
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"fd\":1"));
        assert!(json.contains("\"tid\":\"abc-txn-123\""));
        assert!(json.contains("\"pchain\":\"payment-chain-string\""));
        assert!(json.contains("\"schain\""));
        let decoded: Source = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn source_optional_fields_not_serialised_when_absent() {
        let s = Source {
            fd: Some(0),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(!json.contains("\"tid\""));
        assert!(!json.contains("\"pchain\""));
        assert!(!json.contains("\"schain\""));
        assert!(!json.contains("\"ext\""));
    }

    // ── Regs ──────────────────────────────────────────────────────────────────

    #[test]
    fn regs_empty_roundtrip() {
        let r = Regs::default();
        let json = serde_json::to_string(&r).unwrap();
        assert_eq!(json, "{}");
        let decoded: Regs = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn regs_coppa_roundtrip() {
        let r = Regs {
            coppa: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"coppa\":1"));
        let decoded: Regs = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn regs_gdpr_roundtrip() {
        let r = Regs {
            gdpr: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"gdpr\":1"));
        let decoded: Regs = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn regs_gpp_roundtrip() {
        let r = Regs {
            gpp: Some("DBABMA~CPXxRfAPXxRfA...".to_string()),
            gpp_sid: Some(vec![6]),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"gpp\""));
        assert!(json.contains("\"gpp_sid\":[6]"));
        let decoded: Regs = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn regs_us_privacy_roundtrip() {
        let r = Regs {
            us_privacy: Some("1YNN".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"us_privacy\":\"1YNN\""));
        let decoded: Regs = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn regs_full_roundtrip() {
        let r = Regs {
            coppa: Some(0),
            gdpr: Some(1),
            us_privacy: Some("1YNN".to_string()),
            gpp: Some("DBABMA~CPXxRfA...".to_string()),
            gpp_sid: Some(vec![2, 6]),
            ext: Some(serde_json::json!({ "dsa": {} })),
        };
        let json = serde_json::to_string(&r).unwrap();
        let decoded: Regs = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    // ── DurFloors ─────────────────────────────────────────────────────────────

    #[test]
    fn dur_floors_empty_roundtrip() {
        let d = DurFloors::default();
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(json, "{}");
        let decoded: DurFloors = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn dur_floors_lower_bound_only() {
        let d = DurFloors {
            mindur: Some(31),
            bidfloor: Some(20.0),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"mindur\":31"));
        assert!(json.contains("\"bidfloor\":20.0"));
        assert!(!json.contains("\"maxdur\""));
        let decoded: DurFloors = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn dur_floors_upper_bound_only() {
        let d = DurFloors {
            maxdur: Some(15),
            bidfloor: Some(5.0),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(!json.contains("\"mindur\""));
        assert!(json.contains("\"maxdur\":15"));
        let decoded: DurFloors = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn dur_floors_range_roundtrip() {
        let d = DurFloors {
            mindur: Some(16),
            maxdur: Some(30),
            bidfloor: Some(10.0),
            ext: None,
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"mindur\":16"));
        assert!(json.contains("\"maxdur\":30"));
        assert!(json.contains("\"bidfloor\":10.0"));
        let decoded: DurFloors = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn dur_floors_vec_roundtrip() {
        let floors = vec![
            DurFloors {
                mindur: Some(1),
                maxdur: Some(15),
                bidfloor: Some(5.0),
                ext: None,
            },
            DurFloors {
                mindur: Some(16),
                maxdur: Some(30),
                bidfloor: Some(10.0),
                ext: None,
            },
            DurFloors {
                mindur: Some(31),
                maxdur: None,
                bidfloor: Some(20.0),
                ext: None,
            },
        ];
        let json = serde_json::to_string(&floors).unwrap();
        let decoded: Vec<DurFloors> = serde_json::from_str(&json).unwrap();
        assert_eq!(floors, decoded);
    }

    // ── Qty ───────────────────────────────────────────────────────────────────

    #[test]
    fn qty_minimal_roundtrip() {
        let q = Qty {
            multiplier: 14.2,
            ..Default::default()
        };
        let json = serde_json::to_string(&q).unwrap();
        assert!(json.contains("\"multiplier\":14.2"));
        assert!(!json.contains("\"sourcetype\""));
        assert!(!json.contains("\"vendor\""));
        let decoded: Qty = serde_json::from_str(&json).unwrap();
        assert_eq!(q, decoded);
    }

    #[test]
    fn qty_full_roundtrip() {
        let q = Qty {
            multiplier: 50.0,
            sourcetype: Some(1),
            vendor: Some("measurement-vendor.com".to_string()),
            ext: Some(serde_json::json!({ "panel": "xxx" })),
        };
        let json = serde_json::to_string(&q).unwrap();
        assert!(json.contains("\"multiplier\":50.0"));
        assert!(json.contains("\"sourcetype\":1"));
        assert!(json.contains("\"vendor\":\"measurement-vendor.com\""));
        assert!(json.contains("\"ext\""));
        let decoded: Qty = serde_json::from_str(&json).unwrap();
        assert_eq!(q, decoded);
    }

    #[test]
    fn qty_multiplier_less_than_one() {
        // The spec allows values less than 1 (e.g., fractional impressions).
        let q = Qty {
            multiplier: 0.5,
            ..Default::default()
        };
        let json = serde_json::to_string(&q).unwrap();
        assert!(json.contains("\"multiplier\":0.5"));
        let decoded: Qty = serde_json::from_str(&json).unwrap();
        assert_eq!(q, decoded);
    }
}
