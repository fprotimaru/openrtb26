//! OpenRTB 2.6 implementation in Rust.
//!
//! This crate provides Rust types for the OpenRTB 2.6 specification,
//! with full serde support for JSON serialization and deserialization.
//!
//! # Top-level types
//!
//! - [`BidRequest`] — Section 3.2.1
//! - [`BidResponse`] — Section 4.2.1
//!
//! All subordinate objects are available as public types re-exported from
//! this crate root.

use serde::{Deserialize, Serialize};

// ── modules ──────────────────────────────────────────────────────────────────

pub mod audio;
pub mod banner;
pub mod bid;
pub mod content;
pub mod device;
pub mod eid;
pub mod format;
pub mod geo;
pub mod imp;
pub mod native;
pub mod pmp;
pub mod seat_bid;
pub mod segment;
pub mod site;
pub mod source;
pub mod supply_chain;
pub mod user_agent;
pub mod video;

// ── re-exports ───────────────────────────────────────────────────────────────

pub use audio::Audio;
pub use banner::Banner;
pub use bid::Bid;
pub use content::Content;
pub use device::Device;
pub use eid::{Eid, Uid};
pub use format::Format;
pub use geo::Geo;
pub use imp::{Imp, Metric};
pub use native::Native;
pub use pmp::{Deal, Pmp};
pub use seat_bid::SeatBid;
pub use segment::{Channel, Data, Network, Producer, Publisher, Refresh, RefSettings, Segment, User};
pub use site::{App, Dooh, Site};
pub use source::{DurFloors, Qty, Regs, Source};
pub use supply_chain::{SupplyChain, SupplyChainNode};
pub use user_agent::{BrandVersion, UserAgent};
pub use video::Video;

// ── BidRequest — Section 3.2.1 ──────────────────────────────────────────────

/// Top-level bid request object — Section 3.2.1
///
/// The top-level bid request object contains an exchange-unique bid request or
/// auction ID.  The `id` field and at least one [`Imp`] object are required.
/// Rules and restrictions placed on the top-level object apply to every
/// impression being offered.
///
/// Only one of `site`, `app`, or `dooh` should be present in a single request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BidRequest {
    /// ID of the bid request, assigned by the exchange and unique for tracking.
    pub id: String,

    /// Array of impression objects representing the impressions offered.
    /// At least one `Imp` is required.
    pub imp: Vec<Imp>,

    /// Details about the publisher's website.
    /// Only applicable and recommended for websites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,

    /// Details about the publisher's non-browser application.
    /// Only applicable and recommended for apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,

    /// Details about Digital Out-Of-Home inventory.
    /// A request with a `dooh` object must not contain `site` or `app`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dooh: Option<Dooh>,

    /// Details about the user's device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,

    /// Details about the human user of the device — the advertising audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// Indicator of test mode in which auctions are not billable.
    /// `0` = live mode (default), `1` = test mode.
    #[serde(default, skip_serializing_if = "is_default_test")]
    pub test: i32,

    /// Auction type.
    /// `1` = First Price, `2` = Second Price Plus (default).
    /// Values ≥ 500 are exchange-specific.
    #[serde(default = "default_at", skip_serializing_if = "is_default_at")]
    pub at: i32,

    /// Maximum time in milliseconds the exchange allows for bids to be received,
    /// including Internet latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmax: Option<i32>,

    /// Allowed list of buyer seats (e.g., advertisers, agencies) allowed to bid.
    /// At most one of `wseat` and `bseat` should be used in the same request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,

    /// Block list of buyer seats restricted from bidding.
    /// At most one of `wseat` and `bseat` should be used in the same request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bseat: Option<Vec<String>>,

    /// Flag indicating whether the exchange can verify that all available
    /// impressions in the context are included in this request.
    /// `0` = no or unknown (default), `1` = yes.
    #[serde(default, skip_serializing_if = "is_default_allimps")]
    pub allimps: i32,

    /// Array of allowed currencies for bids (ISO-4217 alpha codes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<Vec<String>>,

    /// Allowed list of creative languages (ISO-639-1-alpha-2).
    /// Only one of `wlang` or `wlangb` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wlang: Option<Vec<String>>,

    /// Allowed list of creative languages (IETF BCP 47).
    /// Only one of `wlang` or `wlangb` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wlangb: Option<Vec<String>>,

    /// Allowed advertiser categories.  Taxonomy defined by `cattax`.
    /// Only one of `acat` or `bcat` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acat: Option<Vec<String>>,

    /// Blocked advertiser categories.  Taxonomy defined by `cattax`.
    /// Only one of `acat` or `bcat` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcat: Option<Vec<String>>,

    /// The taxonomy in use for `bcat` / `acat`.
    /// Refer to AdCOM 1.0 List: Category Taxonomies.
    /// Default `1` = IAB Content Category Taxonomy 1.0.
    #[serde(default = "default_cattax", skip_serializing_if = "is_default_cattax")]
    pub cattax: i32,

    /// Block list of advertisers by their domains (e.g., `"ford.com"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badv: Option<Vec<String>>,

    /// Block list of applications by their app store IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bapp: Option<Vec<String>>,

    /// Data about the inventory source and who makes the final sale decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /// Industry, legal, or governmental regulations in force for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regs: Option<Regs>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── BidResponse — Section 4.2.1 ─────────────────────────────────────────────

/// Top-level bid response object — Section 4.2.1
///
/// The top-level bid response object. The `id` attribute reflects the bid
/// request ID for logging purposes. Similarly, `bidid` is an optional
/// response tracking ID for bidders. If specified, it can be included in the
/// subsequent win notice call if the bidder wins. At least one [`SeatBid`]
/// object is required, which contains at least one bid for an impression.
/// Other attributes are optional.
///
/// To express a "no-bid", the options are to return an empty response with
/// HTTP 204. Alternatively, if the bidder wishes to convey to the exchange a
/// reason for not bidding, just a `BidResponse` object is returned with a
/// reason code in the `nbr` attribute.
///
/// # Example — Ad served on win notice (spec §6.3.1)
/// ```rust
/// use openrtb26::{BidResponse, SeatBid, Bid};
///
/// let response = BidResponse {
///     id: "1234567890".to_string(),
///     bidid: Some("abc1123".to_string()),
///     cur: "USD".to_string(),
///     seatbid: Some(vec![SeatBid {
///         seat: Some("512".to_string()),
///         bid: vec![Bid {
///             id: "1".to_string(),
///             impid: "102".to_string(),
///             price: 9.43,
///             nurl: Some("http://adserver.com/winnotice?impid=102".to_string()),
///             iurl: Some("http://adserver.com/pathtosampleimage".to_string()),
///             adomain: Some(vec!["advertiserdomain.com".to_string()]),
///             cid: Some("campaign111".to_string()),
///             crid: Some("creative112".to_string()),
///             attr: Some(vec![1, 2, 3, 4, 5, 6, 7, 12]),
///             ..Default::default()
///         }],
///         ..Default::default()
///     }]),
///     ..Default::default()
/// };
///
/// let json = serde_json::to_string(&response).unwrap();
/// println!("{json}");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BidResponse {
    /// ID of the bid request to which this is a response. **Required.**
    pub id: String,

    /// Array of [`SeatBid`] objects; 1+ required if a bid is to be made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seatbid: Option<Vec<SeatBid>>,

    /// Bidder-generated response ID to assist with logging/tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidid: Option<String>,

    /// Bid currency using ISO-4217 alpha codes.
    /// Defaults to `"USD"` when not specified.
    #[serde(default = "default_usd", skip_serializing_if = "is_default_usd")]
    pub cur: String,

    /// Optional feature to allow a bidder to set data in the exchange's
    /// cookie. The string must be in base85 cookie safe characters and be in
    /// any format. Proper JSON encoding must be used to include "escaped"
    /// quotation marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,

    /// Reason for not bidding.
    ///
    /// Refer to OpenRTB 3.0 List: No-Bid Reason Codes.
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | 0 | Unknown Error |
    /// | 1 | Technical Error |
    /// | 2 | Invalid Request |
    /// | 3 | Known Web Spider |
    /// | 4 | Suspected Non-Human Traffic |
    /// | 5 | Cloud, Data Center, or Proxy IP |
    /// | 6 | Unsupported Device |
    /// | 7 | Blocked Publisher or Site |
    /// | 8 | Unmatched User |
    /// | 9 | Daily Reader Cap Met |
    /// | 10 | Daily Domain Cap Met |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbr: Option<i32>,

    /// Placeholder for bidder-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

impl Default for BidRequest {
    fn default() -> Self {
        Self {
            id: String::new(),
            imp: vec![],
            site: None,
            app: None,
            dooh: None,
            device: None,
            user: None,
            test: 0,
            at: 2,
            tmax: None,
            wseat: None,
            bseat: None,
            allimps: 0,
            cur: None,
            wlang: None,
            wlangb: None,
            acat: None,
            bcat: None,
            cattax: 1,
            badv: None,
            bapp: None,
            source: None,
            regs: None,
            ext: None,
        }
    }
}

impl Default for BidResponse {
    fn default() -> Self {
        Self {
            id: String::new(),
            seatbid: None,
            bidid: None,
            cur: "USD".to_string(),
            customdata: None,
            nbr: None,
            ext: None,
        }
    }
}

// ── serde helpers ─────────────────────────────────────────────────────────────

fn default_at() -> i32 {
    2
}
fn is_default_test(v: &i32) -> bool {
    *v == 0
}
fn is_default_at(v: &i32) -> bool {
    *v == 2
}
fn is_default_allimps(v: &i32) -> bool {
    *v == 0
}
fn default_cattax() -> i32 {
    1
}
fn is_default_cattax(v: &i32) -> bool {
    *v == 1
}
fn default_usd() -> String {
    "USD".to_string()
}
fn is_default_usd(s: &str) -> bool {
    s == "USD"
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod bid_request_tests {
    use super::*;

    #[test]
    fn bid_request_minimal_roundtrip() {
        let req = BidRequest {
            id: "test-id".to_string(),
            imp: vec![Imp {
                id: "1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&req).unwrap();
        let decoded: BidRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, decoded);
    }

    #[test]
    fn default_at_is_2() {
        let req = BidRequest {
            id: "x".to_string(),
            imp: vec![],
            ..Default::default()
        };
        assert_eq!(req.at, 2);
    }
}

#[cfg(test)]
mod bid_response_tests {
    use super::*;

    #[test]
    fn bid_response_minimal_roundtrip() {
        let r = BidResponse {
            id: "req-123".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"id\":\"req-123\""));
        assert!(!json.contains("\"cur\""));
        let decoded: BidResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn bid_response_no_bid_roundtrip() {
        let r = BidResponse {
            id: "req-no-bid".to_string(),
            nbr: Some(2),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"nbr\":2"));
        assert!(!json.contains("\"seatbid\""));
        let decoded: BidResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn bid_response_default_cur_not_serialised() {
        let r = BidResponse {
            id: "x".to_string(),
            cur: "USD".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(!json.contains("\"cur\""));
    }

    #[test]
    fn bid_response_non_default_cur_serialised() {
        let r = BidResponse {
            id: "x".to_string(),
            cur: "EUR".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"cur\":\"EUR\""));
        let decoded: BidResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn bid_response_customdata_roundtrip() {
        let r = BidResponse {
            id: "x".to_string(),
            customdata: Some("base85cookiedata".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"customdata\":\"base85cookiedata\""));
        let decoded: BidResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn bid_response_with_ext_roundtrip() {
        let r = BidResponse {
            id: "x".to_string(),
            ext: Some(serde_json::json!({ "custom": true })),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"ext\":{\"custom\":true}"));
        let decoded: BidResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn spec_example_1_win_notice() {
        let raw = r#"{
            "id": "1234567890",
            "bidid": "abc1123",
            "cur": "USD",
            "seatbid": [
                {
                    "seat": "512",
                    "bid": [
                        {
                            "id": "1",
                            "impid": "102",
                            "price": 9.43,
                            "nurl": "http://adserver.com/winnotice?impid=102",
                            "iurl": "http://adserver.com/pathtosampleimage",
                            "adomain": ["advertiserdomain.com"],
                            "cid": "campaign111",
                            "crid": "creative112",
                            "attr": [1, 2, 3, 4, 5, 6, 7, 12]
                        }
                    ]
                }
            ]
        }"#;
        let r: BidResponse = serde_json::from_str(raw).unwrap();
        assert_eq!(r.id, "1234567890");
        assert_eq!(r.bidid.as_deref(), Some("abc1123"));
        assert_eq!(r.cur, "USD");

        let seatbids = r.seatbid.as_ref().unwrap();
        assert_eq!(seatbids.len(), 1);
        let sb = &seatbids[0];
        assert_eq!(sb.seat.as_deref(), Some("512"));
        assert_eq!(sb.bid.len(), 1);

        let bid = &sb.bid[0];
        assert_eq!(bid.id, "1");
        assert_eq!(bid.impid, "102");
        assert!((bid.price - 9.43).abs() < f64::EPSILON);
        assert_eq!(
            bid.nurl.as_deref(),
            Some("http://adserver.com/winnotice?impid=102")
        );
        assert_eq!(bid.adomain.as_ref().unwrap()[0], "advertiserdomain.com");
        assert_eq!(bid.cid.as_deref(), Some("campaign111"));
        assert_eq!(bid.crid.as_deref(), Some("creative112"));
        assert_eq!(bid.attr.as_ref().unwrap(), &vec![1, 2, 3, 4, 5, 6, 7, 12]);
    }

    #[test]
    fn spec_example_2_vast_inline() {
        let raw = r#"{
            "id": "123",
            "seatbid": [
                {
                    "bid": [
                        {
                            "id": "12345",
                            "impid": "2",
                            "price": 3.00,
                            "nurl": "http://example.com/winnoticeurl",
                            "adm": "<?xml version=\"1.0\"?><VAST version=\"2.0\"></VAST>"
                        }
                    ]
                }
            ]
        }"#;
        let r: BidResponse = serde_json::from_str(raw).unwrap();
        assert_eq!(r.id, "123");
        let bid = &r.seatbid.as_ref().unwrap()[0].bid[0];
        assert_eq!(bid.id, "12345");
        assert!((bid.price - 3.0).abs() < f64::EPSILON);
        assert!(bid.adm.as_deref().unwrap().contains("VAST"));
    }

    #[test]
    fn spec_example_3_direct_deal() {
        let raw = r#"{
            "id": "1234567890",
            "bidid": "abc1123",
            "cur": "USD",
            "seatbid": [
                {
                    "seat": "512",
                    "bid": [
                        {
                            "id": "1",
                            "impid": "102",
                            "price": 5.00,
                            "dealid": "ABC-1234-6789",
                            "nurl": "http://adserver.com/winnotice?impid=102",
                            "adomain": ["advertiserdomain.com"],
                            "iurl": "http://adserver.com/pathtosampleimage",
                            "cid": "campaign111",
                            "crid": "creative112",
                            "adid": "314",
                            "attr": [1, 2, 3, 4]
                        }
                    ]
                }
            ]
        }"#;
        let r: BidResponse = serde_json::from_str(raw).unwrap();
        let bid = &r.seatbid.as_ref().unwrap()[0].bid[0];
        assert_eq!(bid.dealid.as_deref(), Some("ABC-1234-6789"));
        assert!((bid.price - 5.0).abs() < f64::EPSILON);
        assert_eq!(bid.adid.as_deref(), Some("314"));
    }

    #[test]
    fn spec_example_4_native_inline() {
        let raw = r#"{
            "id": "123",
            "seatbid": [
                {
                    "bid": [
                        {
                            "id": "12345",
                            "impid": "2",
                            "price": 3.00,
                            "nurl": "http://example.com/winnoticeurl",
                            "adm": "{\"native\":{\"ver\":\"1.0\",\"link\":{},\"imptrackers\":[],\"assets\":[]}}"
                        }
                    ]
                }
            ]
        }"#;
        let r: BidResponse = serde_json::from_str(raw).unwrap();
        let bid = &r.seatbid.as_ref().unwrap()[0].bid[0];
        assert!(bid.adm.as_deref().unwrap().contains("native"));
    }

    #[test]
    fn bid_response_full_roundtrip() {
        let r = BidResponse {
            id: "req-full-001".to_string(),
            seatbid: Some(vec![SeatBid {
                bid: vec![
                    Bid {
                        id: "bid-1".to_string(),
                        impid: "imp-1".to_string(),
                        price: 1.5,
                        adomain: Some(vec!["example.com".to_string()]),
                        cid: Some("c1".to_string()),
                        crid: Some("cr1".to_string()),
                        ..Default::default()
                    },
                    Bid {
                        id: "bid-2".to_string(),
                        impid: "imp-2".to_string(),
                        price: 2.0,
                        dealid: Some("deal-xyz".to_string()),
                        ..Default::default()
                    },
                ],
                seat: Some("seat-abc".to_string()),
                group: 0,
                ext: None,
            }]),
            bidid: Some("resp-id-001".to_string()),
            cur: "USD".to_string(),
            customdata: None,
            nbr: None,
            ext: None,
        };
        let json = serde_json::to_string(&r).unwrap();
        let decoded: BidResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded);
        let bids = &decoded.seatbid.as_ref().unwrap()[0].bid;
        assert_eq!(bids.len(), 2);
        assert_eq!(bids[0].id, "bid-1");
        assert_eq!(bids[1].dealid.as_deref(), Some("deal-xyz"));
    }
}
