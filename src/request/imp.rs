//! Imp and Metric objects — Sections 3.2.4 and 3.2.5
//!
//! An `Imp` object describes a single ad placement or impression being
//! auctioned.  A bid request may include multiple `Imp` objects.  Each must
//! carry at least one of `banner`, `video`, `audio`, or `native`.
//!
//! A `Metric` object provides quantifiable, often historical, data points
//! about an impression (e.g. viewability rate, click-through rate).

use serde::{Deserialize, Serialize};

use super::{
    audio::Audio, banner::Banner, native::Native, pmp::Pmp, qty::Qty, refresh::Refresh,
    video::Video,
};

// ── Metric ────────────────────────────────────────────────────────────────────

/// A quantifiable, often historical, data point about an impression — Section 3.2.5
///
/// Metrics are attached to an [`Imp`] as an array and can offer insight to
/// assist with decisioning, such as average recent viewability or
/// click-through rate.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    /// Type of metric being presented using exchange-curated string names
    /// which should be published to bidders a priori.
    #[serde(rename = "type")]
    pub metric_type: String,

    /// Number representing the value of the metric.
    /// Probabilities must be in the range `0.0`–`1.0`.
    pub value: f64,

    /// Source of the value using exchange-curated string names.
    /// If the exchange itself is the source, `"EXCHANGE"` is recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Imp ───────────────────────────────────────────────────────────────────────

/// Ad placement / impression being auctioned — Section 3.2.4
///
/// The `id` field is required so that bids can reference individual
/// impressions.  At least one of `banner`, `video`, `audio`, or `native`
/// must be present to define the type(s) of impression being offered.
/// Any given bid must conform to exactly one of the offered types.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Imp {
    /// Unique identifier for this impression within the bid request.
    /// Typically starts at `"1"` and increments.
    pub id: String,

    /// Array of [`Metric`] objects providing historical data about this
    /// impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<Vec<Metric>>,

    /// Banner ad opportunity.  Required when offering a banner impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Banner>,

    /// Video ad opportunity.  Required when offering a video impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    /// Audio ad opportunity.  Required when offering an audio impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// Native ad opportunity.  Required when offering a native impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<Native>,

    /// Private marketplace deals applicable to this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmp: Option<Pmp>,

    /// Name of ad mediation partner, SDK technology, or player responsible
    /// for rendering the ad.  Recommended for video and/or apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displaymanager: Option<String>,

    /// Version of the ad mediation partner, SDK technology, or player.
    /// Recommended for video and/or apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displaymanagerver: Option<String>,

    /// `1` = interstitial or full-screen ad; `0` = not interstitial (default).
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub instl: i32,

    /// Identifier for the specific ad placement or ad tag that initiated the
    /// auction.  Useful for debugging and buyer-side optimisation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagid: Option<String>,

    /// Minimum bid for this impression expressed in CPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,

    /// Currency of `bidfloor` using ISO-4217 alpha codes.
    /// Defaults to `"USD"` when not specified.
    #[serde(
        default = "default_usd",
        skip_serializing_if = "is_default_bidfloorcur"
    )]
    pub bidfloorcur: String,

    /// Type of browser opened upon clicking the creative in an app.
    /// `0` = embedded, `1` = native.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickbrowser: Option<i32>,

    /// Whether the impression requires secure HTTPS URL creative assets.
    /// `0` = non-secure, `1` = secure.  Omission means unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i32>,

    /// Exchange-specific names of supported iframe busters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframebuster: Option<Vec<String>>,

    /// Whether the user receives a reward for viewing the ad.
    /// `0` = no (default), `1` = yes.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub rwdd: i32,

    /// Server-side ad insertion status and its impact on asset/tracker
    /// retrieval.
    /// `0` = unknown (default), `1` = all client-side, `2` = assets stitched
    /// server-side / pixels client-side, `3` = all server-side.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub ssai: i32,

    /// Advisory number of seconds that may elapse between the auction and the
    /// actual impression being shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i32>,

    /// Impression multiplier for ads that display to more than one person
    /// simultaneously (e.g. DOOH, CTV).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<Qty>,

    /// Estimated fulfilment timestamp in Unix milliseconds (e.g. when a DOOH
    /// impression will be displayed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<f64>,

    /// Details about automatic ad-slot refresh behaviour.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<Refresh>,

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

    #[test]
    fn imp_minimal_roundtrip() {
        let imp = Imp {
            id: "1".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&imp).unwrap();
        let decoded: Imp = serde_json::from_str(&json).unwrap();
        assert_eq!(imp, decoded);
    }

    #[test]
    fn imp_bidfloor_serialised() {
        let imp = Imp {
            id: "1".to_string(),
            bidfloor: Some(0.5),
            bidfloorcur: "EUR".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&imp).unwrap();
        assert!(json.contains("\"bidfloor\":0.5"));
        assert!(json.contains("\"bidfloorcur\":\"EUR\""));
    }

    #[test]
    fn default_bidfloorcur_not_serialised() {
        let imp = Imp {
            id: "1".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&imp).unwrap();
        // "USD" is the default and should be omitted to keep payloads lean
        assert!(!json.contains("bidfloorcur"));
    }

    #[test]
    fn metric_roundtrip() {
        let m = Metric {
            metric_type: "viewability".to_string(),
            value: 0.72,
            vendor: Some("EXCHANGE".to_string()),
            ext: None,
        };
        let json = serde_json::to_string(&m).unwrap();
        let decoded: Metric = serde_json::from_str(&json).unwrap();
        assert_eq!(m, decoded);
    }
}
