//! Video object — Section 3.2.7
//!
//! The `Video` object represents a video impression. Video in OpenRTB generally
//! assumes compliance with the VAST standard. Companion ads are supported via
//! an optional array of [`Banner`] objects.
//!
//! The presence of a `Video` as a subordinate of the `Imp` object indicates
//! that this impression is offered as a video type. The same impression may
//! also be offered as `Banner`, `Audio`, and/or `Native`, but any given bid
//! must conform to exactly one of the offered types.

use serde::{Deserialize, Serialize};

use super::{banner::Banner, dur_floors::DurFloors};

/// Video impression object — Section 3.2.7
///
/// Many fields are non-essential for minimally viable transactions but are
/// included to offer fine control when needed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Video {
    /// Content MIME types supported (e.g., `"video/mp4"`). **Required.**
    pub mimes: Vec<String>,

    /// Minimum video ad duration in seconds.
    /// Mutually exclusive with `rqddurs`; only one may be present.
    /// Default `0` per the spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minduration: Option<i32>,

    /// Maximum video ad duration in seconds.
    /// Mutually exclusive with `rqddurs`; only one may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i32>,

    /// Array of supported video protocols.
    /// Refer to AdCOM 1.0 List: Creative Subtypes - Audio/Video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,

    /// Width of the video player in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height of the video player in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Indicates the start delay in seconds for pre-roll, mid-roll, or
    /// post-roll ad placements.
    /// Refer to AdCOM 1.0 List: Start Delay Modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<i32>,

    /// Video placement type for the impression.
    /// Refer to AdCOM 1.0 List: Plcmt Subtypes - Video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plcmt: Option<i32>,

    /// Indicates if the impression must be linear, nonlinear, etc.
    /// If none specified, assume all are allowed.
    /// Refer to AdCOM 1.0 List: Linearity Modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linearity: Option<i32>,

    /// Indicates if the player will allow the video to be skipped.
    /// `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,

    /// Videos of total duration greater than this number of seconds can be
    /// skippable. Only applicable if the ad is skippable. Default `0`.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub skipmin: i32,

    /// Number of seconds a video must play before skipping is enabled.
    /// Only applicable if the ad is skippable. Default `0`.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub skipafter: i32,

    /// Precise acceptable durations for video creatives in seconds.
    /// Mutually exclusive with `minduration` and `maxduration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rqddurs: Option<Vec<i32>>,

    /// Unique identifier indicating that an impression opportunity belongs to
    /// a video ad pod. Multiple impressions sharing the same `podid` belong to
    /// the same pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub podid: Option<String>,

    /// The sequence (position) of the video ad pod within a content stream.
    /// Refer to AdCOM 1.0 List: Pod Sequence. Default `0`.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub podseq: i32,

    /// Indicates the maximum number of ads that may be served into a "dynamic"
    /// video ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<i32>,

    /// Total amount of time in seconds that advertisers may fill for a
    /// "dynamic" video ad pod, or the dynamic portion of a "hybrid" pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poddur: Option<i32>,

    /// Indicates that the seller can guarantee delivery against the indicated
    /// slot position in the pod.
    /// Refer to AdCOM 1.0 List: Slot Position in Pod. Default `0`.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub slotinpod: i32,

    /// Minimum CPM per second — price floor for the "dynamic" portion of a
    /// video ad pod relative to the bid duration.
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

    /// Indicates if letter-boxing of 4:3 content into a 16:9 window is
    /// allowed. `0` = no, `1` = yes (default).
    #[serde(default = "default_one_i32", skip_serializing_if = "is_one_i32")]
    pub boxingallowed: i32,

    /// Playback methods that may be in use.
    /// Refer to AdCOM 1.0 List: Playback Methods.
    /// Only the first element is typically used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playbackmethod: Option<Vec<i32>>,

    /// The event that causes playback to end.
    /// Refer to AdCOM 1.0 List: Playback Cessation Modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playbackend: Option<i32>,

    /// Supported delivery methods (e.g., streaming, progressive).
    /// If none specified, assume all are supported.
    /// Refer to AdCOM 1.0 List: Delivery Methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<i32>>,

    /// Ad position on screen.
    /// Refer to AdCOM 1.0 List: Placement Positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,

    /// Array of [`Banner`] objects if companion ads are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companionad: Option<Vec<Banner>>,

    /// List of supported API frameworks for this impression.
    /// Refer to AdCOM 1.0 List: API Frameworks.
    /// If an API is not explicitly listed, it is assumed not to be supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Supported VAST companion ad types.
    /// Refer to AdCOM 1.0 List: Companion Types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companiontype: Option<Vec<i32>>,

    /// Pod deduplication settings applied to bid responses.
    /// Refer to AdCOM 1.0 List: Pod Deduplication Settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poddedupe: Option<Vec<i32>>,

    /// Floor prices for video creatives of various durations.
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

fn default_one_i32() -> i32 {
    1
}

fn is_one_i32(v: &i32) -> bool {
    *v == 1
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_minimal_roundtrip() {
        let v = Video {
            mimes: vec!["video/mp4".to_string()],
            ..Default::default()
        };
        let json = serde_json::to_string(&v).unwrap();
        let decoded: Video = serde_json::from_str(&json).unwrap();
        assert_eq!(v, decoded);
    }

    #[test]
    fn boxingallowed_default_not_serialised() {
        let v = Video {
            mimes: vec!["video/mp4".to_string()],
            boxingallowed: 1,
            ..Default::default()
        };
        let json = serde_json::to_string(&v).unwrap();
        assert!(!json.contains("boxingallowed"));
    }

    #[test]
    fn boxingallowed_zero_is_serialised() {
        let v = Video {
            mimes: vec!["video/mp4".to_string()],
            boxingallowed: 0,
            ..Default::default()
        };
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("\"boxingallowed\":0"));
    }

    #[test]
    fn video_full_roundtrip() {
        let v = Video {
            mimes: vec!["video/mp4".to_string(), "video/x-flv".to_string()],
            minduration: Some(5),
            maxduration: Some(30),
            protocols: Some(vec![2, 3]),
            w: Some(640),
            h: Some(480),
            startdelay: Some(0),
            linearity: Some(1),
            skip: Some(1),
            skipafter: 5,
            minbitrate: Some(300),
            maxbitrate: Some(1500),
            boxingallowed: 1,
            playbackmethod: Some(vec![1, 3]),
            delivery: Some(vec![2]),
            pos: Some(1),
            battr: Some(vec![13, 14]),
            api: Some(vec![1, 2]),
            companiontype: Some(vec![1, 2]),
            companionad: Some(vec![Banner {
                id: Some("1234567893-1".to_string()),
                w: Some(300),
                h: Some(250),
                pos: Some(1),
                battr: Some(vec![13, 14]),
                expdir: Some(vec![2, 4]),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let json = serde_json::to_string(&v).unwrap();
        let decoded: Video = serde_json::from_str(&json).unwrap();
        assert_eq!(v, decoded);
    }

    #[test]
    fn pod_fields_roundtrip() {
        let v = Video {
            mimes: vec!["video/mp4".to_string()],
            podid: Some("pod-1".to_string()),
            podseq: 1,
            maxseq: Some(4),
            poddur: Some(60),
            slotinpod: 1,
            mincpmpersec: Some(0.5),
            rqddurs: Some(vec![15, 30]),
            ..Default::default()
        };
        let json = serde_json::to_string(&v).unwrap();
        let decoded: Video = serde_json::from_str(&json).unwrap();
        assert_eq!(v, decoded);
    }

    #[test]
    fn dur_floors_roundtrip() {
        let v = Video {
            mimes: vec!["video/mp4".to_string()],
            durfloors: Some(vec![
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
            ]),
            ..Default::default()
        };
        let json = serde_json::to_string(&v).unwrap();
        let decoded: Video = serde_json::from_str(&json).unwrap();
        assert_eq!(v, decoded);
    }
}
