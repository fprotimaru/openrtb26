//! SeatBid object — Section 4.2.2
//!
//! A bid response can contain multiple `SeatBid` objects, each on behalf of a
//! different bidder seat and each containing one or more individual bids. If
//! multiple impressions are presented in the request, the `group` attribute
//! can be used to specify if a seat is willing to accept any impressions that
//! it can win (default) or if it is only interested in winning any if it can
//! win them all as a group.

use serde::{Deserialize, Serialize};

use super::bid::Bid;

/// A collection of bids made by the bidder on behalf of a specific seat
/// — Section 4.2.2
///
/// A seat represents an advertising entity (e.g., advertiser, agency) that
/// wishes to obtain impressions and uses bidders to act on their behalf.
///
/// # Example
/// ```rust
/// use openrtb26::{SeatBid, Bid};
///
/// let seatbid = SeatBid {
///     bid: vec![Bid {
///         id: "1".to_string(),
///         impid: "102".to_string(),
///         price: 9.43,
///         nurl: Some("http://adserver.com/winnotice?impid=102".to_string()),
///         ..Default::default()
///     }],
///     seat: Some("512".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SeatBid {
    /// Array of 1+ [`Bid`] objects each related to an impression.
    /// Multiple bids can relate to the same impression.
    ///
    /// **Required.**
    pub bid: Vec<Bid>,

    /// ID of the buyer seat (e.g., advertiser, agency) on whose behalf this
    /// bid is made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<String>,

    /// Indicates whether impressions must be won or lost as a group.
    ///
    /// `0` = impressions can be won individually (default),
    /// `1` = impressions must be won or lost as a group.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub group: i32,

    /// Placeholder for bidder-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── serde helpers ─────────────────────────────────────────────────────────────

fn is_zero_i32(v: &i32) -> bool {
    *v == 0
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seatbid_minimal_roundtrip() {
        let sb = SeatBid {
            bid: vec![Bid {
                id: "1".to_string(),
                impid: "102".to_string(),
                price: 9.43,
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&sb).unwrap();
        assert!(json.contains("\"bid\""));
        assert!(json.contains("\"id\":\"1\""));
        assert!(json.contains("\"impid\":\"102\""));
        assert!(json.contains("\"price\":9.43"));
        // Default fields should not be serialised
        assert!(!json.contains("\"seat\""));
        assert!(!json.contains("\"group\""));
        assert!(!json.contains("\"ext\""));
        let decoded: SeatBid = serde_json::from_str(&json).unwrap();
        assert_eq!(sb, decoded);
    }

    #[test]
    fn seatbid_with_seat_roundtrip() {
        let sb = SeatBid {
            bid: vec![Bid {
                id: "b1".to_string(),
                impid: "i1".to_string(),
                price: 1.0,
                ..Default::default()
            }],
            seat: Some("seat-abc".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&sb).unwrap();
        assert!(json.contains("\"seat\":\"seat-abc\""));
        let decoded: SeatBid = serde_json::from_str(&json).unwrap();
        assert_eq!(sb, decoded);
    }

    #[test]
    fn seatbid_group_default_not_serialised() {
        let sb = SeatBid {
            bid: vec![Bid {
                id: "b1".to_string(),
                impid: "i1".to_string(),
                price: 1.0,
                ..Default::default()
            }],
            group: 0,
            ..Default::default()
        };
        let json = serde_json::to_string(&sb).unwrap();
        assert!(!json.contains("\"group\""));
    }

    #[test]
    fn seatbid_group_one_is_serialised() {
        let sb = SeatBid {
            bid: vec![Bid {
                id: "b1".to_string(),
                impid: "i1".to_string(),
                price: 1.0,
                ..Default::default()
            }],
            group: 1,
            ..Default::default()
        };
        let json = serde_json::to_string(&sb).unwrap();
        assert!(json.contains("\"group\":1"));
        let decoded: SeatBid = serde_json::from_str(&json).unwrap();
        assert_eq!(sb, decoded);
    }

    #[test]
    fn seatbid_multiple_bids_roundtrip() {
        let sb = SeatBid {
            bid: vec![
                Bid {
                    id: "bid-1".to_string(),
                    impid: "imp-1".to_string(),
                    price: 1.5,
                    adomain: Some(vec!["advertiser1.com".to_string()]),
                    crid: Some("cr-001".to_string()),
                    ..Default::default()
                },
                Bid {
                    id: "bid-2".to_string(),
                    impid: "imp-2".to_string(),
                    price: 2.75,
                    dealid: Some("deal-xyz".to_string()),
                    mtype: Some(2),
                    ..Default::default()
                },
            ],
            seat: Some("dsp-seat-1".to_string()),
            group: 0,
            ext: None,
        };
        let json = serde_json::to_string(&sb).unwrap();
        assert!(json.contains("\"bid-1\""));
        assert!(json.contains("\"bid-2\""));
        assert!(json.contains("\"deal-xyz\""));
        assert!(json.contains("\"dsp-seat-1\""));
        let decoded: SeatBid = serde_json::from_str(&json).unwrap();
        assert_eq!(sb, decoded);
        assert_eq!(decoded.bid.len(), 2);
        assert_eq!(decoded.bid[0].id, "bid-1");
        assert!((decoded.bid[0].price - 1.5).abs() < f64::EPSILON);
        assert_eq!(decoded.bid[1].dealid.as_deref(), Some("deal-xyz"));
        assert_eq!(decoded.bid[1].mtype, Some(2));
    }

    #[test]
    fn seatbid_with_ext_roundtrip() {
        let sb = SeatBid {
            bid: vec![Bid {
                id: "b1".to_string(),
                impid: "i1".to_string(),
                price: 0.5,
                ..Default::default()
            }],
            ext: Some(serde_json::json!({ "custom_seat_data": "value" })),
            ..Default::default()
        };
        let json = serde_json::to_string(&sb).unwrap();
        assert!(json.contains("\"ext\":{\"custom_seat_data\":\"value\"}"));
        let decoded: SeatBid = serde_json::from_str(&json).unwrap();
        assert_eq!(sb, decoded);
    }

    #[test]
    fn seatbid_group_parsed_from_raw_json() {
        let raw = r#"{
            "bid": [
                { "id": "b1", "impid": "i1", "price": 3.5 },
                { "id": "b2", "impid": "i2", "price": 2.0 }
            ],
            "seat": "seat-group-test",
            "group": 1
        }"#;
        let sb: SeatBid = serde_json::from_str(raw).unwrap();
        assert_eq!(sb.group, 1);
        assert_eq!(sb.seat.as_deref(), Some("seat-group-test"));
        assert_eq!(sb.bid.len(), 2);
        assert_eq!(sb.bid[0].id, "b1");
        assert!((sb.bid[1].price - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn seatbid_empty_bid_array_is_valid_struct() {
        // While the spec says 1+ bids are required, the struct itself allows
        // an empty vec — callers are responsible for populating it correctly.
        let sb = SeatBid {
            bid: vec![],
            ..Default::default()
        };
        let json = serde_json::to_string(&sb).unwrap();
        assert!(json.contains("\"bid\":[]"));
        let decoded: SeatBid = serde_json::from_str(&json).unwrap();
        assert_eq!(sb, decoded);
    }
}
