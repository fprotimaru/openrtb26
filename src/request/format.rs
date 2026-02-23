//! Format object — Section 3.2.10
//!
//! This object represents an allowed size (height and width combination) or
//! Flex Ad parameters for a banner impression.  These are typically used in
//! an array where multiple sizes are permitted.
//!
//! It is recommended that either the `w`/`h` pair **or** the
//! `wratio`/`hratio`/`wmin` set (for Flex Ads) be specified.

use serde::{Deserialize, Serialize};

/// Allowed banner size or Flex Ad parameters — Section 3.2.10
///
/// Used inside [`Banner::format`](super::banner::Banner::format) to enumerate
/// every permitted creative dimension for a given banner impression.
///
/// # Fixed-size example
/// ```rust
/// use openrtb26::Format;
///
/// let f = Format { w: Some(300), h: Some(250), ..Default::default() };
/// ```
///
/// # Flex-Ad example
/// ```rust
/// use openrtb26::Format;
///
/// let f = Format {
///     wratio: Some(4),
///     hratio: Some(3),
///     wmin: Some(300),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Format {
    /// Width in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Relative width when expressing size as a ratio (Flex Ads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// Relative height when expressing size as a ratio (Flex Ads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// Minimum width in device-independent pixels (DIPS) at which the ad will
    /// be displayed when the size is expressed as a ratio (Flex Ads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_size_roundtrip() {
        let f = Format {
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };
        let json = serde_json::to_string(&f).unwrap();
        assert_eq!(json, r#"{"w":300,"h":250}"#);
        let decoded: Format = serde_json::from_str(&json).unwrap();
        assert_eq!(f, decoded);
    }

    #[test]
    fn flex_ad_roundtrip() {
        let f = Format {
            wratio: Some(4),
            hratio: Some(3),
            wmin: Some(300),
            ..Default::default()
        };
        let json = serde_json::to_string(&f).unwrap();
        assert!(json.contains("\"wratio\":4"));
        assert!(json.contains("\"hratio\":3"));
        assert!(json.contains("\"wmin\":300"));
        assert!(!json.contains("\"w\""));
        assert!(!json.contains("\"h\""));
        let decoded: Format = serde_json::from_str(&json).unwrap();
        assert_eq!(f, decoded);
    }

    #[test]
    fn empty_format_serialises_to_empty_object() {
        let f = Format::default();
        let json = serde_json::to_string(&f).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn with_ext_roundtrip() {
        let f = Format {
            w: Some(160),
            h: Some(600),
            ext: Some(serde_json::json!({ "custom_key": "custom_value" })),
            ..Default::default()
        };
        let json = serde_json::to_string(&f).unwrap();
        assert!(json.contains("\"ext\""));
        let decoded: Format = serde_json::from_str(&json).unwrap();
        assert_eq!(f, decoded);
    }
}
