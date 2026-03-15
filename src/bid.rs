//! Bid object — Section 4.2.3
//!
//! A `SeatBid` object contains one or more `Bid` objects, each of which
//! relates to a specific impression in the bid request via the `impid`
//! attribute and constitutes an offer to buy that impression for a given
//! `price`.

use serde::{Deserialize, Serialize};

/// An offer to buy a specific impression — Section 4.2.3
///
/// A `Bid` object relates to a specific impression in the bid request via the
/// `impid` attribute and constitutes an offer to buy that impression for a
/// given `price`.
///
/// # Ad Serving
///
/// There are two standard methods for transferring markup from the bidder to
/// the exchange:
///
/// 1. **Markup in the bid** — populate the `adm` field. If both `adm` and the
///    win notice return data, the `adm` contents take precedence.
/// 2. **Markup on the win notice** — omit `adm`; the response body of the
///    win notice call (i.e., invoking `nurl`) contains the ad markup and only
///    the ad markup.
///
/// # Substitution Macros
///
/// The `nurl`, `burl`, `lurl`, and `adm` fields may contain substitution
/// macros (Section 4.4) that the exchange will replace before calling the URLs
/// or delivering the markup. Standard macros include:
///
/// - `${AUCTION_ID}` — ID of the bid request
/// - `${AUCTION_BID_ID}` — ID of the bid (from `BidResponse.bidid`)
/// - `${AUCTION_IMP_ID}` — ID of the impression won
/// - `${AUCTION_SEAT_ID}` — ID of the bidder seat
/// - `${AUCTION_AD_ID}` — ID of the ad markup (`bid.adid`)
/// - `${AUCTION_PRICE}` — Clearing price in the same currency and units as the bid
/// - `${AUCTION_CURRENCY}` — Currency of the bid
/// - `${AUCTION_MBR}` — Market Bid Ratio (clearance price / bid price)
/// - `${AUCTION_LOSS}` — Loss reason codes
/// - `${AUCTION_MIN_TO_WIN}` — Minimum bid required to win
/// - `${AUCTION_MULTIPLIER}` — Quantity of impressions won
/// - `${AUCTION_IMP_TS}` — Timestamp when the impression was fulfilled (Unix ms)
///
/// # Example
/// ```rust
/// use openrtb26::Bid;
///
/// let bid = Bid {
///     id: "1".to_string(),
///     impid: "102".to_string(),
///     price: 9.43,
///     nurl: Some("http://adserver.com/winnotice?impid=102&price=${AUCTION_PRICE}".to_string()),
///     adomain: Some(vec!["advertiserdomain.com".to_string()]),
///     iurl: Some("http://adserver.com/pathtosampleimage".to_string()),
///     cid: Some("campaign111".to_string()),
///     crid: Some("creative112".to_string()),
///     attr: Some(vec![1, 2, 3, 4, 5, 6, 7, 12]),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bid {
    /// Bidder-generated bid ID to assist with logging/tracking.
    ///
    /// **Required.**
    pub id: String,

    /// ID of the [`Imp`](crate::Imp) object in the related bid request.
    ///
    /// **Required.**
    pub impid: String,

    /// Bid price expressed as CPM although the actual transaction is for a
    /// unit impression only.
    ///
    /// Note that while the type indicates float, integer math is highly
    /// recommended when handling currencies (e.g., `BigDecimal` in Java or
    /// using fixed-point arithmetic).
    ///
    /// **Required.**
    pub price: f64,

    /// Win notice URL called by the exchange if the bid wins (not necessarily
    /// indicative of a delivered, viewed, or billable ad); optional means of
    /// serving ad markup.
    ///
    /// Substitution macros (Section 4.4) may be included in both the URL and
    /// optionally returned markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,

    /// Billing notice URL called by the exchange when a winning bid becomes
    /// billable based on exchange-specific business policy (e.g., typically
    /// delivered, viewed, etc.).
    ///
    /// Substitution macros (Section 4.4) may be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burl: Option<String>,

    /// Loss notice URL called by the exchange when a bid is known to have been
    /// lost.
    ///
    /// Substitution macros (Section 4.4) may be included. Exchange-specific
    /// policy may preclude support for loss notices or the disclosure of
    /// winning clearing prices resulting in `${AUCTION_PRICE}` macros being
    /// removed (i.e., replaced with a zero-length string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurl: Option<String>,

    /// Optional means of conveying ad markup in case the bid wins; supersedes
    /// the win notice if markup is included in both.
    ///
    /// Substitution macros (Section 4.4) may be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// ID of a preloaded ad to be served if the bid wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adid: Option<String>,

    /// Advertiser domain for block list checking (e.g., `"ford.com"`).
    ///
    /// This can be an array for the case of rotating creatives. Exchanges can
    /// mandate that only one domain is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adomain: Option<Vec<String>>,

    /// The store ID of the app in an app store (e.g., Apple App Store, Google
    /// Play).
    ///
    /// See [OTT/CTV Store Assigned App Identification Guidelines](https://iabtechlab.com/wp-content/uploads/2020/08/IAB-Tech-Lab-OTT-store-assigned-App-Identification-Guidelines-2020.pdf)
    /// for more details about expected strings for CTV app stores.
    ///
    /// For mobile apps in Google Play Store, these should be bundle or package
    /// names (e.g., `"com.foo.mygame"`). For apps in Apple App Store, these
    /// should be a numeric ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    /// URL without cache-busting to an image that is representative of the
    /// content of the campaign for ad quality/safety checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iurl: Option<String>,

    /// Campaign ID to assist with ad quality checking; the collection of
    /// creatives for which `iurl` should be representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,

    /// Creative ID to assist with ad quality checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crid: Option<String>,

    /// Tactic ID to enable buyers to label bids for reporting to the exchange
    /// the tactic through which their bid was submitted.
    ///
    /// The specific usage and meaning of the tactic ID should be communicated
    /// between buyer and exchanges a priori.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tactic: Option<String>,

    /// The taxonomy in use for `cat`.
    ///
    /// Refer to AdCOM 1.0 List: Category Taxonomies.
    /// Default `1` = IAB Content Category Taxonomy 1.0.
    #[serde(default = "default_cattax", skip_serializing_if = "is_default_cattax")]
    pub cattax: i32,

    /// IAB Tech Lab content categories of the creative.
    ///
    /// The taxonomy to be used is defined by the `cattax` field.
    /// If no `cattax` field is supplied, Content Taxonomy 1.0 is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Set of attributes describing the creative.
    ///
    /// Refer to AdCOM 1.0 List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<Vec<i32>>,

    /// List of supported APIs for the markup.
    ///
    /// If an API is not explicitly listed, it is assumed to be unsupported.
    /// Refer to AdCOM 1.0 List: API Frameworks.
    ///
    /// Replaces the deprecated `api` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<i32>>,

    /// **DEPRECATED in favour of `apis`.**
    /// Single API framework integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<i32>,

    /// Video response protocol of the markup if applicable.
    ///
    /// Refer to AdCOM 1.0 List: Creative Subtypes - Audio/Video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<i32>,

    /// Creative media rating per IQG guidelines.
    ///
    /// Refer to AdCOM 1.0 List: Media Ratings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<i32>,

    /// Language of the creative using ISO-639-1-alpha-2.
    ///
    /// The non-standard code `"xx"` may also be used if the creative has no
    /// linguistic content (e.g., a banner with just a company logo).
    ///
    /// Only one of `language` or `langb` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Language of the creative using IETF BCP 47.
    ///
    /// Only one of `language` or `langb` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langb: Option<String>,

    /// Reference to the `deal.id` from the bid request if this bid pertains to
    /// a private marketplace direct deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dealid: Option<String>,

    /// Width of the creative in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height of the creative in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Relative width of the creative when expressing size as a ratio.
    /// Required for Flex Ads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// Relative height of the creative when expressing size as a ratio.
    /// Required for Flex Ads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// Advisory as to the number of seconds the bidder is willing to wait
    /// between the auction and the actual impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i32>,

    /// Duration of the video or audio creative in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dur: Option<i32>,

    /// Type of the creative markup so that it can be properly associated with
    /// the right sub-object of the `BidRequest.Imp`.
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | 1 | Banner |
    /// | 2 | Video |
    /// | 3 | Audio |
    /// | 4 | Native |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtype: Option<i32>,

    /// Indicates that the bid response is only eligible for a specific position
    /// within a video or audio ad pod (e.g., first position, last position, or
    /// any).
    ///
    /// Refer to AdCOM 1.0 List: Slot Position in Pod. Default `0`.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub slotinpod: i32,

    /// Placeholder for bidder-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

impl Default for Bid {
    fn default() -> Self {
        Self {
            id: String::new(),
            impid: String::new(),
            price: 0.0,
            nurl: None,
            burl: None,
            lurl: None,
            adm: None,
            adid: None,
            adomain: None,
            bundle: None,
            iurl: None,
            cid: None,
            crid: None,
            tactic: None,
            cattax: 1,
            cat: None,
            attr: None,
            apis: None,
            api: None,
            protocol: None,
            qagmediarating: None,
            language: None,
            langb: None,
            dealid: None,
            w: None,
            h: None,
            wratio: None,
            hratio: None,
            exp: None,
            dur: None,
            mtype: None,
            slotinpod: 0,
            ext: None,
        }
    }
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

    #[test]
    fn bid_minimal_roundtrip() {
        let bid = Bid {
            id: "1".to_string(),
            impid: "102".to_string(),
            price: 9.43,
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"id\":\"1\""));
        assert!(json.contains("\"impid\":\"102\""));
        assert!(json.contains("\"price\":9.43"));
        // Default/absent fields must not appear
        assert!(!json.contains("\"nurl\""));
        assert!(!json.contains("\"adm\""));
        assert!(!json.contains("\"cattax\""));
        assert!(!json.contains("\"slotinpod\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_with_nurl_roundtrip() {
        let bid = Bid {
            id: "b1".to_string(),
            impid: "i1".to_string(),
            price: 1.5,
            nurl: Some("http://adserver.com/win?price=${AUCTION_PRICE}".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"nurl\""));
        assert!(json.contains("${AUCTION_PRICE}"));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_with_adm_roundtrip() {
        let bid = Bid {
            id: "b2".to_string(),
            impid: "i2".to_string(),
            price: 3.0,
            adm: Some("<vast/>".to_string()),
            mtype: Some(2),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"adm\":\"<vast/>\""));
        assert!(json.contains("\"mtype\":2"));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_with_burl_lurl_roundtrip() {
        let bid = Bid {
            id: "b3".to_string(),
            impid: "i3".to_string(),
            price: 2.5,
            burl: Some("http://adserver.com/billing?price=${AUCTION_PRICE}".to_string()),
            lurl: Some("http://adserver.com/loss?reason=${AUCTION_LOSS}".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"burl\""));
        assert!(json.contains("\"lurl\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_deal_roundtrip() {
        let bid = Bid {
            id: "1".to_string(),
            impid: "102".to_string(),
            price: 5.0,
            dealid: Some("ABC-1234-6789".to_string()),
            adomain: Some(vec!["advertiserdomain.com".to_string()]),
            iurl: Some("http://adserver.com/pathtosampleimage".to_string()),
            cid: Some("campaign111".to_string()),
            crid: Some("creative112".to_string()),
            adid: Some("314".to_string()),
            attr: Some(vec![1, 2, 3, 4]),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"dealid\":\"ABC-1234-6789\""));
        assert!(json.contains("\"adid\":\"314\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_default_cattax_not_serialised() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            cattax: 1,
            cat: Some(vec!["IAB1".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(!json.contains("\"cattax\""));
        assert!(json.contains("\"cat\":[\"IAB1\"]"));
    }

    #[test]
    fn bid_non_default_cattax_serialised() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            cattax: 3,
            cat: Some(vec!["1".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"cattax\":3"));
    }

    #[test]
    fn bid_slotinpod_default_not_serialised() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            slotinpod: 0,
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(!json.contains("\"slotinpod\""));
    }

    #[test]
    fn bid_slotinpod_non_zero_serialised() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            slotinpod: 1,
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"slotinpod\":1"));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_mtype_values_roundtrip() {
        for mtype in [1, 2, 3, 4] {
            let bid = Bid {
                id: "b".to_string(),
                impid: "i".to_string(),
                price: 1.0,
                mtype: Some(mtype),
                ..Default::default()
            };
            let json = serde_json::to_string(&bid).unwrap();
            assert!(json.contains(&format!("\"mtype\":{mtype}")));
            let decoded: Bid = serde_json::from_str(&json).unwrap();
            assert_eq!(bid, decoded);
        }
    }

    #[test]
    fn bid_dimensions_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            w: Some(300),
            h: Some(250),
            wratio: Some(4),
            hratio: Some(3),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"w\":300"));
        assert!(json.contains("\"h\":250"));
        assert!(json.contains("\"wratio\":4"));
        assert!(json.contains("\"hratio\":3"));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_language_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            language: Some("en".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"language\":\"en\""));
        assert!(!json.contains("\"langb\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_langb_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            langb: Some("zh-Hans".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"langb\":\"zh-Hans\""));
        assert!(!json.contains("\"language\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_apis_replaces_deprecated_api() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            apis: Some(vec![1, 2, 7]),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"apis\":[1,2,7]"));
        // Deprecated singular 'api' field must not be auto-populated
        assert!(!json.contains("\"api\":"));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_deprecated_api_field_still_parses() {
        // Backwards compatibility: older responses may carry the deprecated
        // singular `api` field.
        let raw = r#"{"id":"b","impid":"i","price":1.0,"api":3}"#;
        let bid: Bid = serde_json::from_str(raw).unwrap();
        assert_eq!(bid.api, Some(3));
        assert!(bid.apis.is_none());
    }

    #[test]
    fn bid_dur_exp_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            dur: Some(30),
            exp: Some(120),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"dur\":30"));
        assert!(json.contains("\"exp\":120"));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_tactic_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            tactic: Some("prospecting".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"tactic\":\"prospecting\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_bundle_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            bundle: Some("com.example.app".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"bundle\":\"com.example.app\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_protocol_qagmediarating_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            protocol: Some(2),
            qagmediarating: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"protocol\":2"));
        assert!(json.contains("\"qagmediarating\":1"));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_with_ext_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            ext: Some(serde_json::json!({ "dsp_data": "custom_value", "priority": 1 })),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"ext\""));
        assert!(json.contains("\"dsp_data\":\"custom_value\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    #[test]
    fn bid_adomain_multiple_values_roundtrip() {
        let bid = Bid {
            id: "b".to_string(),
            impid: "i".to_string(),
            price: 1.0,
            adomain: Some(vec![
                "brand.com".to_string(),
                "brand-subdomain.brand.com".to_string(),
            ]),
            ..Default::default()
        };
        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"brand.com\""));
        assert!(json.contains("\"brand-subdomain.brand.com\""));
        let decoded: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, decoded);
    }

    /// Replicates spec Example 1 — Ad Served on Win Notice (§6.3.1).
    #[test]
    fn spec_example_1_bid_from_json() {
        let raw = r#"{
            "id": "1",
            "impid": "102",
            "price": 9.43,
            "nurl": "http://adserver.com/winnotice?impid=102",
            "iurl": "http://adserver.com/pathtosampleimage",
            "adomain": ["advertiserdomain.com"],
            "cid": "campaign111",
            "crid": "creative112",
            "attr": [1, 2, 3, 4, 5, 6, 7, 12]
        }"#;
        let bid: Bid = serde_json::from_str(raw).unwrap();
        assert_eq!(bid.id, "1");
        assert_eq!(bid.impid, "102");
        assert!((bid.price - 9.43).abs() < f64::EPSILON);
        assert_eq!(
            bid.nurl.as_deref(),
            Some("http://adserver.com/winnotice?impid=102")
        );
        assert_eq!(
            bid.iurl.as_deref(),
            Some("http://adserver.com/pathtosampleimage")
        );
        assert_eq!(bid.adomain.as_ref().unwrap()[0], "advertiserdomain.com");
        assert_eq!(bid.cid.as_deref(), Some("campaign111"));
        assert_eq!(bid.crid.as_deref(), Some("creative112"));
        assert_eq!(
            bid.attr.as_ref().unwrap(),
            &vec![1i32, 2, 3, 4, 5, 6, 7, 12]
        );
    }

    /// Replicates spec Example 2 — VAST XML returned inline (§6.3.2).
    #[test]
    fn spec_example_2_vast_bid_from_json() {
        let raw = r#"{
            "id": "12345",
            "impid": "2",
            "price": 3.00,
            "nurl": "http://example.com/winnoticeurl",
            "adm": "<?xml version=\"1.0\" encoding=\"utf-8\"?><VAST version=\"2.0\"></VAST>"
        }"#;
        let bid: Bid = serde_json::from_str(raw).unwrap();
        assert_eq!(bid.id, "12345");
        assert!((bid.price - 3.0).abs() < f64::EPSILON);
        assert!(bid.adm.as_deref().unwrap().contains("VAST"));
    }
}
