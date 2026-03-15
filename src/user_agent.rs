//! UserAgent and BrandVersion objects — Sections 3.2.29 and 3.2.30
//!
//! Structured user agent information, which can be used when a client supports
//! [User-Agent Client Hints](https://wicg.github.io/ua-client-hints/).
//!
//! If both `device.ua` and `device.sua` are present in the bid request,
//! `device.sua` should be considered the more accurate representation of the
//! device attributes. This is because `device.ua` may contain a frozen or
//! reduced user agent string due to deprecation of user agent strings by
//! browsers.

use serde::{Deserialize, Serialize};

// ── BrandVersion ──────────────────────────────────────────────────────────────

/// Browser or platform brand and version — Section 3.2.30
///
/// Based on [User-Agent Client Hints](https://wicg.github.io/ua-client-hints/),
/// the `BrandVersion` object is used to identify a device's browser or similar
/// software component, and the user agent's execution platform or operating
/// system.
///
/// # Examples
///
/// Browser brand (from `Sec-CH-UA-Full-Version-List`):
/// ```rust
/// use openrtb26::BrandVersion;
///
/// let chrome = BrandVersion {
///     brand: "Google Chrome".to_string(),
///     version: Some(vec![
///         "114".to_string(),
///         "0".to_string(),
///         "5735".to_string(),
///         "90".to_string(),
///     ]),
///     ext: None,
/// };
/// ```
///
/// Platform brand (from `Sec-CH-UA-Platform` + `Sec-CH-UA-Platform-Version`):
/// ```rust
/// use openrtb26::BrandVersion;
///
/// let windows = BrandVersion {
///     brand: "Windows".to_string(),
///     version: Some(vec!["10".to_string(), "0".to_string()]),
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BrandVersion {
    /// A brand identifier, for example `"Chrome"` or `"Windows"`.
    ///
    /// The value may be sourced from the User-Agent Client Hints headers,
    /// representing either the user agent brand (from `Sec-CH-UA-Full-Version-List`)
    /// or the platform brand (from `Sec-CH-UA-Platform`).
    ///
    /// **Required.**
    pub brand: String,

    /// A sequence of version components in descending hierarchical order
    /// (major, minor, micro, …).
    ///
    /// For example `["114", "0", "5735", "90"]` for Chrome 114.0.5735.90.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Vec<String>>,

    /// Placeholder for vendor-specific extensions to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── UserAgent ─────────────────────────────────────────────────────────────────

/// Structured user agent information — Section 3.2.29
///
/// This object carries structured user agent data derived from
/// [User-Agent Client Hints](https://wicg.github.io/ua-client-hints/) headers
/// or the equivalent JavaScript `NavigatorUAData` interface.
///
/// When both [`Device::ua`](super::device::Device::ua) and
/// [`Device::sua`](super::device::Device::sua) are present in the bid request,
/// `sua` should be considered the more accurate representation of the device
/// attributes, because `ua` may contain a frozen or reduced user agent string.
///
/// # Example
/// ```rust
/// use openrtb26::{UserAgent, BrandVersion};
///
/// let sua = UserAgent {
///     browsers: Some(vec![
///         BrandVersion {
///             brand: "Google Chrome".to_string(),
///             version: Some(vec!["114".to_string()]),
///             ext: None,
///         },
///         BrandVersion {
///             brand: "Chromium".to_string(),
///             version: Some(vec!["114".to_string()]),
///             ext: None,
///         },
///     ]),
///     platform: Some(BrandVersion {
///         brand: "Windows".to_string(),
///         version: Some(vec!["10".to_string(), "0".to_string()]),
///         ext: None,
///     }),
///     mobile: Some(0),
///     architecture: Some("x86".to_string()),
///     bitness: Some("64".to_string()),
///     model: None,
///     source: 1,
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserAgent {
    /// Each [`BrandVersion`] object identifies a browser or similar software
    /// component.
    ///
    /// Implementers should send brands and versions derived from the
    /// `Sec-CH-UA-Full-Version-List` header (or the equivalent
    /// `NavigatorUAData` accessor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browsers: Option<Vec<BrandVersion>>,

    /// A [`BrandVersion`] object that identifies the user agent's execution
    /// platform / OS.
    ///
    /// Implementers should send a brand derived from `Sec-CH-UA-Platform` and
    /// a version derived from `Sec-CH-UA-Platform-Version`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<BrandVersion>,

    /// `1` if the agent prefers a "mobile" version of the content (optimised
    /// for small screens or touch input), `0` if the agent prefers the
    /// "desktop" or "full" content.
    ///
    /// Implementers should derive this value from the `Sec-CH-UA-Mobile`
    /// header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i32>,

    /// Device's major binary architecture, e.g. `"x86"` or `"arm"`.
    ///
    /// Implementers should retrieve this value from the `Sec-CH-UA-Arch`
    /// header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Device's bitness, e.g. `"64"` for a 64-bit architecture.
    ///
    /// Implementers should retrieve this value from the `Sec-CH-UA-Bitness`
    /// header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<String>,

    /// Device model.
    ///
    /// Implementers should retrieve this value from the `Sec-CH-UA-Model`
    /// header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The source of data used to create this object.
    ///
    /// Refer to AdCOM 1.0 List: User-Agent Source.
    /// Default `0` (unknown / not specified).
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub source: i32,

    /// Placeholder for vendor-specific extensions to this object.
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

    // ── BrandVersion ─────────────────────────────────────────────────────────

    #[test]
    fn brand_version_minimal_roundtrip() {
        let bv = BrandVersion {
            brand: "Chrome".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&bv).unwrap();
        assert_eq!(json, r#"{"brand":"Chrome"}"#);
        let decoded: BrandVersion = serde_json::from_str(&json).unwrap();
        assert_eq!(bv, decoded);
    }

    #[test]
    fn brand_version_with_version_roundtrip() {
        let bv = BrandVersion {
            brand: "Google Chrome".to_string(),
            version: Some(vec![
                "114".to_string(),
                "0".to_string(),
                "5735".to_string(),
                "90".to_string(),
            ]),
            ext: None,
        };
        let json = serde_json::to_string(&bv).unwrap();
        assert!(json.contains("\"brand\":\"Google Chrome\""));
        assert!(json.contains("\"version\":[\"114\",\"0\",\"5735\",\"90\"]"));
        let decoded: BrandVersion = serde_json::from_str(&json).unwrap();
        assert_eq!(bv, decoded);
    }

    #[test]
    fn brand_version_with_ext_roundtrip() {
        let bv = BrandVersion {
            brand: "Windows".to_string(),
            version: Some(vec!["10".to_string()]),
            ext: Some(serde_json::json!({ "custom": true })),
        };
        let json = serde_json::to_string(&bv).unwrap();
        assert!(json.contains("\"ext\":{\"custom\":true}"));
        let decoded: BrandVersion = serde_json::from_str(&json).unwrap();
        assert_eq!(bv, decoded);
    }

    #[test]
    fn brand_version_no_version_field_when_none() {
        let bv = BrandVersion {
            brand: "Not A;Brand".to_string(),
            version: None,
            ext: None,
        };
        let json = serde_json::to_string(&bv).unwrap();
        assert!(!json.contains("\"version\""));
        assert!(!json.contains("\"ext\""));
    }

    // ── UserAgent ─────────────────────────────────────────────────────────────

    #[test]
    fn user_agent_empty_roundtrip() {
        let ua = UserAgent::default();
        let json = serde_json::to_string(&ua).unwrap();
        // All optional fields and the default-zero source should be absent.
        assert_eq!(json, "{}");
        let decoded: UserAgent = serde_json::from_str(&json).unwrap();
        assert_eq!(ua, decoded);
    }

    #[test]
    fn user_agent_source_zero_not_serialised() {
        let ua = UserAgent {
            source: 0,
            ..Default::default()
        };
        let json = serde_json::to_string(&ua).unwrap();
        assert!(!json.contains("\"source\""));
    }

    #[test]
    fn user_agent_non_zero_source_serialised() {
        let ua = UserAgent {
            source: 1,
            ..Default::default()
        };
        let json = serde_json::to_string(&ua).unwrap();
        assert!(json.contains("\"source\":1"));
    }

    #[test]
    fn user_agent_browsers_roundtrip() {
        let ua = UserAgent {
            browsers: Some(vec![
                BrandVersion {
                    brand: "Google Chrome".to_string(),
                    version: Some(vec!["114".to_string()]),
                    ext: None,
                },
                BrandVersion {
                    brand: "Chromium".to_string(),
                    version: Some(vec!["114".to_string()]),
                    ext: None,
                },
                BrandVersion {
                    brand: "Not A;Brand".to_string(),
                    version: Some(vec!["99".to_string()]),
                    ext: None,
                },
            ]),
            ..Default::default()
        };
        let json = serde_json::to_string(&ua).unwrap();
        assert!(json.contains("\"browsers\""));
        assert!(json.contains("\"Google Chrome\""));
        assert!(json.contains("\"Chromium\""));
        assert!(json.contains("\"Not A;Brand\""));
        let decoded: UserAgent = serde_json::from_str(&json).unwrap();
        assert_eq!(ua, decoded);
    }

    #[test]
    fn user_agent_platform_roundtrip() {
        let ua = UserAgent {
            platform: Some(BrandVersion {
                brand: "macOS".to_string(),
                version: Some(vec!["13".to_string(), "4".to_string(), "1".to_string()]),
                ext: None,
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&ua).unwrap();
        assert!(json.contains("\"platform\""));
        assert!(json.contains("\"macOS\""));
        assert!(json.contains("[\"13\",\"4\",\"1\"]"));
        let decoded: UserAgent = serde_json::from_str(&json).unwrap();
        assert_eq!(ua, decoded);
    }

    #[test]
    fn user_agent_full_roundtrip() {
        let ua = UserAgent {
            browsers: Some(vec![
                BrandVersion {
                    brand: "Google Chrome".to_string(),
                    version: Some(vec![
                        "114".to_string(),
                        "0".to_string(),
                        "5735".to_string(),
                        "90".to_string(),
                    ]),
                    ext: None,
                },
                BrandVersion {
                    brand: "Chromium".to_string(),
                    version: Some(vec![
                        "114".to_string(),
                        "0".to_string(),
                        "5735".to_string(),
                        "90".to_string(),
                    ]),
                    ext: None,
                },
            ]),
            platform: Some(BrandVersion {
                brand: "Windows".to_string(),
                version: Some(vec!["10".to_string(), "0".to_string()]),
                ext: None,
            }),
            mobile: Some(0),
            architecture: Some("x86".to_string()),
            bitness: Some("64".to_string()),
            model: Some("".to_string()),
            source: 1,
            ext: None,
        };
        let json = serde_json::to_string(&ua).unwrap();
        assert!(json.contains("\"mobile\":0"));
        assert!(json.contains("\"architecture\":\"x86\""));
        assert!(json.contains("\"bitness\":\"64\""));
        assert!(json.contains("\"source\":1"));
        let decoded: UserAgent = serde_json::from_str(&json).unwrap();
        assert_eq!(ua, decoded);
    }

    #[test]
    fn user_agent_mobile_zero_is_serialised_when_explicitly_set() {
        // mobile=0 (desktop) is meaningful and must be included when set.
        let ua = UserAgent {
            mobile: Some(0),
            ..Default::default()
        };
        let json = serde_json::to_string(&ua).unwrap();
        assert!(json.contains("\"mobile\":0"));
    }

    #[test]
    fn user_agent_mobile_one_is_serialised() {
        let ua = UserAgent {
            mobile: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&ua).unwrap();
        assert!(json.contains("\"mobile\":1"));
    }

    #[test]
    fn user_agent_parse_from_raw_json() {
        let raw = r#"{
            "browsers": [
                { "brand": "Chromium", "version": ["110", "0"] },
                { "brand": "Not A;Brand", "version": ["99"] }
            ],
            "platform": { "brand": "Linux", "version": ["5", "15"] },
            "mobile": 0,
            "architecture": "arm",
            "bitness": "64",
            "source": 2
        }"#;
        let ua: UserAgent = serde_json::from_str(raw).unwrap();
        let browsers = ua.browsers.as_ref().unwrap();
        assert_eq!(browsers.len(), 2);
        assert_eq!(browsers[0].brand, "Chromium");
        assert_eq!(
            browsers[0].version.as_ref().unwrap(),
            &vec!["110".to_string(), "0".to_string()]
        );
        let platform = ua.platform.as_ref().unwrap();
        assert_eq!(platform.brand, "Linux");
        assert_eq!(ua.mobile, Some(0));
        assert_eq!(ua.architecture.as_deref(), Some("arm"));
        assert_eq!(ua.bitness.as_deref(), Some("64"));
        assert_eq!(ua.source, 2);
    }
}
