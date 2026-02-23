//! Audio object — Section 3.2.8
//!
//! The `Audio` object represents an audio type impression. Audio in OpenRTB
//! generally assumes compliance with the VAST standard. Companion ads are
//! supported via an optional array of [`Banner`] objects.
//!
//! The presence of an `Audio` as a subordinate of the `Imp` object indicates
//! that this impression is offered as an audio type. The same impression may
//! also be offered as `Banner`, `Video`, and/or `Native`, but any given bid
//! must conform to exactly one of the offered types.

use serde::{Deserialize, Serialize};

use super::{banner::Banner, dur_floors::DurFloors};

/// Audio impression object — Section 3.2.8
///
/// Many fields are non-essential for minimally viable transactions but are
/// included to offer fine control when needed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    /// Content MIME types supported (e.g., `"audio/mp4"`). **Required.**
    pub mimes: Vec<String>,

    /// Minimum audio ad duration in seconds.
    /// Mutually exclusive with `rqddurs`; only one may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minduration: Option<i32>,

    /// Maximum audio ad duration in seconds.
    /// Mutually exclusive with `rqddurs`; only one may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i32>,

    /// Total amount of time in seconds that advertisers may fill for a
    /// "dynamic" audio ad pod, or the dynamic portion of a "hybrid" pod.
    /// Required only for the dynamic portion(s) of audio ad pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poddur: Option<i32>,

    /// Array of supported audio protocols.
    /// Refer to AdCOM 1.0 List: Creative Subtypes - Audio/Video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,

    /// Indicates the start delay in seconds for pre-roll, mid-roll, or
    /// post-roll ad placements.
    /// Refer to AdCOM 1.0 List: Start Delay Modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<i32>,

    /// Precise acceptable durations for audio creatives in seconds.
    /// Targets the live audio/radio use case where non-exact durations would
    /// result in undesirable dead air.
    /// Mutually exclusive with `minduration` and `maxduration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rqddurs: Option<Vec<i32>>,

    /// Unique identifier indicating that an impression opportunity belongs to
    /// an audio ad pod. Multiple impressions sharing the same `podid` belong
    /// to the same audio ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub podid: Option<String>,

    /// The sequence (position) of the audio ad pod within a content stream.
    /// Refer to AdCOM 1.0 List: Pod Sequence. Default `0`.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub podseq: i32,

    /// For audio ad pods, indicates that the seller can guarantee delivery
    /// against the indicated slot position in the pod.
    /// Refer to AdCOM 1.0 List: Slot Position in Pod. Default `0`.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub slotinpod: i32,

    /// Minimum CPM per second — price floor for the "dynamic" portion of an
    /// audio ad pod relative to the bid duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mincpmpersec: Option<f64>,

    /// Blocked creative attributes.
    /// Refer to AdCOM 1.0 List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,

    /// Maximum extended ad duration if extension is allowed.
    /// `0` or absent = not allowed; `-1` = allowed with no time limit;
    /// `> 0` = number of additional seconds beyond `maxduration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxextended: Option<i32>,

    /// Minimum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i32>,

    /// Maximum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i32>,

    /// Supported delivery methods (e.g., streaming, progressive).
    /// If none specified, assume all are supported.
    /// Refer to AdCOM 1.0 List: Delivery Methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<i32>>,

    /// Array of [`Banner`] objects if companion ads are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companionad: Option<Vec<Banner>>,

    /// List of supported API frameworks for this impression.
    /// Refer to AdCOM 1.0 List: API Frameworks.
    /// If an API is not explicitly listed, it is assumed not to be supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Supported companion ad types.
    /// Refer to AdCOM 1.0 List: Companion Types.
    /// Recommended if companion `Banner` objects are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companiontype: Option<Vec<i32>>,

    /// The maximum number of ads that can be played in an ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<i32>,

    /// Type of audio feed.
    /// Refer to AdCOM 1.0 List: Feed Types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<i32>,

    /// Indicates if the ad is stitched with audio content or delivered
    /// independently. `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stitched: Option<i32>,

    /// Volume normalization mode.
    /// Refer to AdCOM 1.0 List: Volume Normalization Modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvol: Option<i32>,

    /// Floor prices for audio creatives of various durations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durfloors: Option<Vec<DurFloors>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
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
    fn audio_minimal_roundtrip() {
        let a = Audio {
            mimes: vec!["audio/mp4".to_string()],
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        let decoded: Audio = serde_json::from_str(&json).unwrap();
        assert_eq!(a, decoded);
    }

    #[test]
    fn audio_full_roundtrip() {
        let a = Audio {
            mimes: vec!["audio/mp4".to_string(), "audio/ogg".to_string()],
            minduration: Some(15),
            maxduration: Some(30),
            poddur: Some(90),
            protocols: Some(vec![2, 3, 6]),
            startdelay: Some(0),
            rqddurs: None,
            podid: Some("pod-abc".to_string()),
            podseq: 1,
            slotinpod: 2,
            mincpmpersec: Some(0.25),
            battr: Some(vec![1, 2]),
            maxextended: Some(10),
            minbitrate: Some(128),
            maxbitrate: Some(320),
            delivery: Some(vec![1, 2]),
            companionad: Some(vec![Banner {
                w: Some(300),
                h: Some(250),
                ..Default::default()
            }]),
            api: Some(vec![3, 5]),
            companiontype: Some(vec![1, 2]),
            maxseq: Some(3),
            feed: Some(1),
            stitched: Some(1),
            nvol: Some(1),
            durfloors: Some(vec![DurFloors {
                mindur: Some(1),
                maxdur: Some(15),
                bidfloor: Some(5.0),
                ext: None,
            }]),
            ext: None,
        };
        let json = serde_json::to_string(&a).unwrap();
        let decoded: Audio = serde_json::from_str(&json).unwrap();
        assert_eq!(a, decoded);
    }

    #[test]
    fn default_fields_not_serialised() {
        let a = Audio {
            mimes: vec!["audio/mp4".to_string()],
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        // default i32 = 0 should not appear
        assert!(!json.contains("\"podseq\""));
        assert!(!json.contains("\"slotinpod\""));
        // optional fields should not appear
        assert!(!json.contains("\"minduration\""));
        assert!(!json.contains("\"maxduration\""));
    }

    #[test]
    fn non_zero_podseq_is_serialised() {
        let a = Audio {
            mimes: vec!["audio/mp4".to_string()],
            podseq: 2,
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        assert!(json.contains("\"podseq\":2"));
    }

    #[test]
    fn rqddurs_mutually_exclusive_example() {
        // When rqddurs is set, minduration / maxduration should not be set.
        // This test ensures both serialise independently as the spec demands;
        // enforcement of mutual exclusivity is left to the caller.
        let a = Audio {
            mimes: vec!["audio/mp4".to_string()],
            rqddurs: Some(vec![15, 30, 60]),
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        assert!(json.contains("\"rqddurs\":[15,30,60]"));
        assert!(!json.contains("minduration"));
        assert!(!json.contains("maxduration"));
        let decoded: Audio = serde_json::from_str(&json).unwrap();
        assert_eq!(a, decoded);
    }
}
