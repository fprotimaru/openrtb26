//! Banner object — Section 3.2.6
//!
//! The `Banner` object represents the most general type of impression.
//! It can be a simple static image, an expandable ad unit, or even
//! in-banner video.  An array of `Banner` objects can also appear within
//! a `Video` object to describe optional VAST companion ads.

use serde::{Deserialize, Serialize};

use super::format::Format;

/// Banner impression object — Section 3.2.6
///
/// The presence of a `Banner` as a subordinate of the [`Imp`](super::super::imp::Imp)
/// object indicates that this impression is offered as a banner type.
/// At the publisher's discretion the same impression may also be offered
/// as video, audio, and/or native by including the respective objects.
/// However, any given bid must conform to exactly one of the offered types.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Banner {
    /// Array of [`Format`] objects representing the banner sizes permitted.
    /// If none are specified then use of `w` and `h` is highly recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Vec<Format>>,

    /// Exact width in device-independent pixels (DIPS).
    /// Recommended if no `Format` objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Exact height in device-independent pixels (DIPS).
    /// Recommended if no `Format` objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Blocked banner ad types.
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | 1 | XHTML Text Ad |
    /// | 2 | XHTML Banner Ad |
    /// | 3 | JavaScript Ad |
    /// | 4 | iframe |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<Vec<i32>>,

    /// Blocked creative attributes.
    /// Refer to AdCOM 1.0 List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,

    /// Ad position on screen.
    /// Refer to AdCOM 1.0 List: Placement Positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,

    /// Content MIME types supported (e.g., `"image/jpeg"`, `"image/gif"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,

    /// Indicates if the banner is in the top frame as opposed to an iframe.
    /// `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topframe: Option<i32>,

    /// Directions in which the banner may expand.
    /// Refer to AdCOM 1.0 List: Expandable Directions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expdir: Option<Vec<i32>>,

    /// List of supported API frameworks for this impression.
    /// Refer to AdCOM 1.0 List: API Frameworks.
    /// If an API is not explicitly listed, it is assumed not to be supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Unique identifier for this banner object.
    /// Recommended when `Banner` objects are used with a `Video` object to
    /// represent an array of companion ads.  Values usually start at `"1"` and
    /// increase with each object; should be unique within an impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Companion banner rendering mode relative to the associated video.
    /// Only relevant when the `Banner` is used inside a `Video` companion-ad
    /// array.
    /// `0` = concurrent (default), `1` = end-card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcm: Option<i32>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_minimal_roundtrip() {
        let banner = Banner {
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        let decoded: Banner = serde_json::from_str(&json).unwrap();
        assert_eq!(banner, decoded);
    }

    #[test]
    fn banner_with_format_roundtrip() {
        let banner = Banner {
            format: Some(vec![
                Format {
                    w: Some(300),
                    h: Some(250),
                    ..Default::default()
                },
                Format {
                    w: Some(728),
                    h: Some(90),
                    ..Default::default()
                },
            ]),
            pos: Some(1),
            battr: Some(vec![13, 14]),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        let decoded: Banner = serde_json::from_str(&json).unwrap();
        assert_eq!(banner, decoded);
    }

    #[test]
    fn banner_companion_fields() {
        let companion = Banner {
            id: Some("1234567893-1".to_string()),
            w: Some(300),
            h: Some(250),
            pos: Some(1),
            battr: Some(vec![13, 14]),
            expdir: Some(vec![2, 4]),
            vcm: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&companion).unwrap();
        assert!(json.contains("\"id\":\"1234567893-1\""));
        assert!(json.contains("\"vcm\":1"));
        assert!(json.contains("\"expdir\":[2,4]"));
        let decoded: Banner = serde_json::from_str(&json).unwrap();
        assert_eq!(companion, decoded);
    }

    #[test]
    fn empty_banner_has_no_extra_fields() {
        let banner = Banner::default();
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, "{}");
    }
}
