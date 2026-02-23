//! Native object — Section 3.2.9
//!
//! The `Native` object represents a native type impression. Native ad units
//! are intended to blend seamlessly into the surrounding content (e.g., a
//! sponsored Twitter or Facebook post).
//!
//! The Native Subcommittee has developed a companion specification to OpenRTB
//! called the Dynamic Native Ads API. It defines the request parameters and
//! response markup structure of native ad units. This object provides the
//! means of transporting request parameters as an opaque string so that the
//! specific parameters can evolve separately under the auspices of that API.
//!
//! The presence of a `Native` as a subordinate of the `Imp` object indicates
//! that this impression is offered as a native type. The same impression may
//! also be offered as `Banner`, `Video`, and/or `Audio`, but any given bid
//! must conform to exactly one of the offered types.

use serde::{Deserialize, Serialize};

/// Native impression object — Section 3.2.9
///
/// The `request` field carries the serialised Native Ad Specification payload.
/// Its exact structure is defined by the Dynamic Native Ads API and evolves
/// independently of OpenRTB.
///
/// # OpenRTB Native 1.0
/// The payload is a JSON-encoded string consisting of an unnamed root object
/// with a single subordinate object named `"native"` which is the Native
/// Markup Request Object (section 4.1 of the Native 1.0 spec).
///
/// # OpenRTB Native 1.1+
/// The payload is a JSON-encoded string consisting of an unnamed root object
/// which is itself the Native Markup Request Object.
///
/// # Example (Native 1.1+)
/// ```rust
/// use openrtb26::Native;
///
/// let native = Native {
///     request: r#"{"ver":"1.1","layout":1,"adunit":2}"#.to_string(),
///     ver: Some("1.1".to_string()),
///     ..Default::default()
/// };
///
/// let json = serde_json::to_string(&native).unwrap();
/// println!("{json}");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Native {
    /// Request payload complying with the Native Ad Specification.
    ///
    /// For Native 1.0 this is a JSON-encoded string with a root object
    /// containing a `"native"` key.  For Native 1.1+ it is a JSON-encoded
    /// string whose root object **is** the Native Markup Request Object.
    ///
    /// **Required.**
    pub request: String,

    /// Version of the Dynamic Native Ads API to which `request` complies.
    /// Highly recommended for efficient parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// List of supported API frameworks for this impression.
    /// Refer to AdCOM 1.0 List: API Frameworks.
    /// If an API is not explicitly listed it is assumed not to be supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Blocked creative attributes.
    /// Refer to AdCOM 1.0 List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_minimal_roundtrip() {
        let n = Native {
            request: r#"{"ver":"1.1","layout":1}"#.to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&n).unwrap();
        let decoded: Native = serde_json::from_str(&json).unwrap();
        assert_eq!(n, decoded);
    }

    #[test]
    fn native_with_ver_roundtrip() {
        let n = Native {
            request: r#"{"ver":"1.1","layout":1,"adunit":2}"#.to_string(),
            ver: Some("1.1".to_string()),
            api: Some(vec![3, 5]),
            battr: Some(vec![1, 2, 3]),
            ext: None,
        };
        let json = serde_json::to_string(&n).unwrap();
        assert!(json.contains("\"ver\":\"1.1\""));
        assert!(json.contains("\"api\":[3,5]"));
        assert!(json.contains("\"battr\":[1,2,3]"));
        let decoded: Native = serde_json::from_str(&json).unwrap();
        assert_eq!(n, decoded);
    }

    #[test]
    fn native_empty_optional_fields_not_serialised() {
        let n = Native {
            request: "{}".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&n).unwrap();
        assert!(!json.contains("\"ver\""));
        assert!(!json.contains("\"api\""));
        assert!(!json.contains("\"battr\""));
        assert!(!json.contains("\"ext\""));
    }

    #[test]
    fn native_with_ext_roundtrip() {
        let n = Native {
            request: "{}".to_string(),
            ver: Some("1.2".to_string()),
            ext: Some(serde_json::json!({ "custom": true })),
            ..Default::default()
        };
        let json = serde_json::to_string(&n).unwrap();
        assert!(json.contains("\"ext\":{\"custom\":true}"));
        let decoded: Native = serde_json::from_str(&json).unwrap();
        assert_eq!(n, decoded);
    }

    /// Ensure the `request` field is treated as a raw string, not a nested
    /// JSON object — OpenRTB mandates the entire Native payload is an opaque
    /// JSON-encoded string inside the surrounding bid-request JSON.
    #[test]
    fn request_field_is_a_string() {
        let n = Native {
            request: r#"{"native":{"ver":"1.0"}}"#.to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&n).unwrap();
        // The value of "request" must be a JSON string, not an embedded object.
        assert!(json.contains(r#""request":"{\"native\":"#));
    }
}
