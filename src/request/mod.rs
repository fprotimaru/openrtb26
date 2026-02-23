//! Bid Request specification — Section 3
//!
//! All objects that form a bid request as defined in the OpenRTB 2.6 spec.

use serde::{Deserialize, Serialize};

pub mod app;
pub mod audio;
pub mod banner;
pub mod brand_version;
pub mod channel;
pub mod content;
pub mod data;
pub mod deal;
pub mod device;
pub mod dooh;
pub mod dur_floors;
pub mod eid;
pub mod format;
pub mod geo;
pub mod imp;
pub mod native;
pub mod network;
pub mod pmp;
pub mod producer;
pub mod publisher;
pub mod qty;
pub mod refresh;
pub mod refsettings;
pub mod regs;
pub mod segment;
pub mod site;
pub mod source;
pub mod supply_chain;
pub mod uid;
pub mod user;
pub mod user_agent;
pub mod video;

use app::App;
use device::Device;
use dooh::Dooh;
use imp::Imp;
use regs::Regs;
use site::Site;
use source::Source;
use user::User;

/// Top-level bid request object — Section 3.2.1
///
/// The top-level bid request object contains an exchange-unique bid request or
/// auction ID.  The `id` field and at least one [`Imp`] object are required.
/// Rules and restrictions placed on the top-level object apply to every
/// impression being offered.
///
/// Only one of `site`, `app`, or `dooh` should be present in a single request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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

// ── helpers for serde default/skip ───────────────────────────────────────────

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

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::imp::Imp;

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
