//! Pmp and Deal objects — Sections 3.2.11 and 3.2.12
//!
//! The `Pmp` object is the private marketplace container for direct deals
//! between buyers and sellers that may pertain to an impression.
//!
//! The `Deal` object constitutes a specific deal that was struck between a
//! buyer and a seller. Its presence within the `Pmp` collection indicates
//! that this impression is available under the terms of that deal.

use serde::{Deserialize, Serialize};

use super::source::DurFloors;

// ── Deal ──────────────────────────────────────────────────────────────────────

/// A specific deal struck between a buyer and a seller — Section 3.2.12
///
/// Its presence within the [`Pmp`] collection indicates that this impression
/// is available under the terms of that deal.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Deal {
    /// A unique identifier for the direct deal. **Required.**
    pub id: String,

    /// Minimum bid for this impression expressed in CPM.
    /// Default `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,

    /// Currency specified using ISO-4217 alpha codes.
    /// This field does not inherit from `Imp.bidfloorcur`; it is either
    /// explicitly specified or defaults to `"USD"`.
    #[serde(
        default = "default_usd",
        skip_serializing_if = "is_default_bidfloorcur"
    )]
    pub bidfloorcur: String,

    /// Optional override of the overall auction type of the bid request.
    /// `1` = First Price, `2` = Second Price Plus,
    /// `3` = the value passed in `bidfloor` is the agreed upon deal price.
    /// Additional auction types can be defined by the exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<i32>,

    /// Allowed list of buyer seats (e.g., advertisers, agencies) that may bid
    /// on this deal.  Omission implies no seat restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,

    /// Array of advertiser domains allowed to bid on this deal.
    /// Omission implies no advertiser restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wadomain: Option<Vec<String>>,

    /// Indicates that the deal is of type `guaranteed` and the bidder must bid
    /// on it. `0` = not a guaranteed deal (default), `1` = guaranteed deal.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub guar: i32,

    /// Minimum CPM per second — price floor for video or audio impression
    /// opportunities, relative to the duration of bids an advertiser may
    /// submit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mincpmpersec: Option<f64>,

    /// Container for floor price by duration information, to be used if a
    /// given deal is eligible for video or audio demand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durfloors: Option<Vec<DurFloors>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Pmp ───────────────────────────────────────────────────────────────────────

/// Private marketplace container for direct deals — Section 3.2.11
///
/// This object is the private marketplace container for direct deals between
/// buyers and sellers that may pertain to a given impression.
/// The actual deals are represented as a collection of [`Deal`] objects.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Pmp {
    /// Indicator of auction eligibility to seats named in the Direct Deals
    /// object.
    /// `0` = all bids are accepted (default),
    /// `1` = bids are restricted to the deals specified and the terms thereof.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub private_auction: i32,

    /// Array of [`Deal`] objects that convey the specific deals applicable to
    /// this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deals: Option<Vec<Deal>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── serde helpers ─────────────────────────────────────────────────────────────

fn default_usd() -> String {
    "USD".to_string()
}

fn is_default_bidfloorcur(s: &str) -> bool {
    s == "USD"
}

fn is_zero_i32(v: &i32) -> bool {
    *v == 0
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Deal tests ────────────────────────────────────────────────────────────

    #[test]
    fn deal_minimal_roundtrip() {
        let deal = Deal {
            id: "AB-Agency1-0001".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&deal).unwrap();
        let decoded: Deal = serde_json::from_str(&json).unwrap();
        assert_eq!(deal, decoded);
    }

    #[test]
    fn deal_default_bidfloorcur_not_serialised() {
        let deal = Deal {
            id: "d1".to_string(),
            bidfloorcur: "USD".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&deal).unwrap();
        assert!(!json.contains("bidfloorcur"));
    }

    #[test]
    fn deal_non_default_bidfloorcur_serialised() {
        let deal = Deal {
            id: "d1".to_string(),
            bidfloorcur: "EUR".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&deal).unwrap();
        assert!(json.contains("\"bidfloorcur\":\"EUR\""));
    }

    #[test]
    fn deal_full_roundtrip() {
        let deal = Deal {
            id: "XY-Agency2-0001".to_string(),
            bidfloor: Some(2.5),
            bidfloorcur: "USD".to_string(),
            at: Some(2),
            wseat: Some(vec!["Agency2".to_string()]),
            wadomain: Some(vec!["advertiser.com".to_string()]),
            guar: 1,
            mincpmpersec: Some(0.1),
            durfloors: Some(vec![DurFloors {
                mindur: Some(1),
                maxdur: Some(15),
                bidfloor: Some(5.0),
                ext: None,
            }]),
            ext: None,
        };
        let json = serde_json::to_string(&deal).unwrap();
        let decoded: Deal = serde_json::from_str(&json).unwrap();
        assert_eq!(deal, decoded);
    }

    #[test]
    fn deal_guar_default_not_serialised() {
        let deal = Deal {
            id: "d1".to_string(),
            guar: 0,
            ..Default::default()
        };
        let json = serde_json::to_string(&deal).unwrap();
        assert!(!json.contains("\"guar\""));
    }

    #[test]
    fn deal_guar_one_is_serialised() {
        let deal = Deal {
            id: "d1".to_string(),
            guar: 1,
            ..Default::default()
        };
        let json = serde_json::to_string(&deal).unwrap();
        assert!(json.contains("\"guar\":1"));
    }

    // ── Pmp tests ─────────────────────────────────────────────────────────────

    #[test]
    fn pmp_empty_roundtrip() {
        let pmp = Pmp::default();
        let json = serde_json::to_string(&pmp).unwrap();
        assert_eq!(json, "{}");
        let decoded: Pmp = serde_json::from_str(&json).unwrap();
        assert_eq!(pmp, decoded);
    }

    #[test]
    fn pmp_private_auction_default_not_serialised() {
        let pmp = Pmp {
            private_auction: 0,
            ..Default::default()
        };
        let json = serde_json::to_string(&pmp).unwrap();
        assert!(!json.contains("private_auction"));
    }

    #[test]
    fn pmp_private_auction_one_is_serialised() {
        let pmp = Pmp {
            private_auction: 1,
            ..Default::default()
        };
        let json = serde_json::to_string(&pmp).unwrap();
        assert!(json.contains("\"private_auction\":1"));
    }

    #[test]
    fn pmp_with_deals_roundtrip() {
        let pmp = Pmp {
            private_auction: 1,
            deals: Some(vec![
                Deal {
                    id: "AB-Agency1-0001".to_string(),
                    at: Some(1),
                    bidfloor: Some(2.5),
                    wseat: Some(vec!["Agency1".to_string()]),
                    ..Default::default()
                },
                Deal {
                    id: "XY-Agency2-0001".to_string(),
                    at: Some(2),
                    bidfloor: Some(2.0),
                    wseat: Some(vec!["Agency2".to_string()]),
                    ..Default::default()
                },
            ]),
            ext: None,
        };
        let json = serde_json::to_string(&pmp).unwrap();
        assert!(json.contains("\"private_auction\":1"));
        assert!(json.contains("\"AB-Agency1-0001\""));
        assert!(json.contains("\"XY-Agency2-0001\""));
        let decoded: Pmp = serde_json::from_str(&json).unwrap();
        assert_eq!(pmp, decoded);
    }

    /// Verify the spec Example 5 — PMP with Direct Deal structure.
    #[test]
    fn spec_example_5_pmp_structure() {
        let pmp_json = r#"{
            "private_auction": 1,
            "deals": [
                {
                    "id": "AB-Agency1-0001",
                    "at": 1,
                    "bidfloor": 2.5,
                    "wseat": ["Agency1"]
                },
                {
                    "id": "XY-Agency2-0001",
                    "at": 2,
                    "bidfloor": 2.0,
                    "wseat": ["Agency2"]
                }
            ]
        }"#;
        let pmp: Pmp = serde_json::from_str(pmp_json).unwrap();
        assert_eq!(pmp.private_auction, 1);
        let deals = pmp.deals.as_ref().unwrap();
        assert_eq!(deals.len(), 2);
        assert_eq!(deals[0].id, "AB-Agency1-0001");
        assert_eq!(deals[0].at, Some(1));
        assert_eq!(deals[0].bidfloor, Some(2.5));
        assert_eq!(deals[1].id, "XY-Agency2-0001");
        assert_eq!(deals[1].at, Some(2));
    }
}
