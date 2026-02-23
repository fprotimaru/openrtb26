//! Device object — Section 3.2.18
//!
//! This object provides information pertaining to the device through which the
//! user is interacting. Device information includes its hardware, platform,
//! location, and carrier data. The device can refer to a mobile handset, a
//! desktop computer, set top box, or other digital device.

use serde::{Deserialize, Serialize};

use super::{geo::Geo, user_agent::UserAgent};

/// Device information — Section 3.2.18
///
/// # Deprecation notice
///
/// The following fields were deprecated as of OpenRTB 2.6 and are retained
/// only for backwards compatibility with older supply sources. New
/// implementations should not populate them:
///
/// - `didsha1`
/// - `didmd5`
/// - `dpidsha1`
/// - `dpidmd5`
/// - `macsha1`
/// - `macmd5`
///
/// # User-Agent guidance
///
/// When a client supports User-Agent Client Hints and both `ua` and `sua` are
/// present, `sua` should be considered the more accurate representation of the
/// device attributes, because `ua` may contain a frozen or reduced user agent
/// string.
///
/// # Example
/// ```rust
/// use openrtb26::Device;
///
/// let device = Device {
///     ua: Some("Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X)".to_string()),
///     make: Some("Apple".to_string()),
///     model: Some("iPhone".to_string()),
///     os: Some("iOS".to_string()),
///     osv: Some("16.0".to_string()),
///     devicetype: Some(4),
///     ifa: Some("AA000DFE74168477C70D291f574D344790E0BB11".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Device {
    /// Location of the device, assumed to be the user's current location,
    /// defined by a [`Geo`] object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    /// Standard "Do Not Track" flag as set in the header by the browser.
    /// `0` = tracking is unrestricted, `1` = do not track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnt: Option<i32>,

    /// "Limit Ad Tracking" signal commercially endorsed (e.g., iOS, Android).
    /// `0` = tracking is unrestricted, `1` = tracking must be limited per
    /// commercial guidelines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<i32>,

    /// Browser user agent string.
    ///
    /// This field represents a raw user agent string from the browser. For
    /// backwards compatibility, exchanges are recommended to always populate
    /// `ua` with the User-Agent string when available from the end user's
    /// device, even if `sua` is also populated. No inferred or approximated
    /// user agents are expected in this field.
    ///
    /// If a client supports User-Agent Client Hints and `sua` is present,
    /// bidders are recommended to rely on `sua` for device/browser detection
    /// and ignore `ua`, which may be frozen or reduced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<String>,

    /// Structured user agent information derived from User-Agent Client Hints.
    ///
    /// If both `ua` and `sua` are present, `sua` should be considered the more
    /// accurate representation of the device attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sua: Option<UserAgent>,

    /// IPv4 address closest to the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// IP address closest to the device as IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,

    /// The general type of device.
    ///
    /// Refer to AdCOM 1.0 List: Device Types.
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | 1 | Mobile / Tablet |
    /// | 2 | Personal Computer |
    /// | 3 | Connected TV |
    /// | 4 | Phone |
    /// | 5 | Tablet |
    /// | 6 | Connected Device |
    /// | 7 | Set Top Box |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicetype: Option<i32>,

    /// Device make / manufacturer (e.g., `"Apple"`, `"Samsung"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,

    /// Device model (e.g., `"iPhone"`, `"Galaxy S23"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Device operating system (e.g., `"iOS"`, `"Android"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    /// Device operating system version (e.g., `"16.0"`, `"13"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osv: Option<String>,

    /// Hardware version of the device (e.g., `"5S"` for iPhone 5S).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwv: Option<String>,

    /// Physical height of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Physical width of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Screen size as pixels per linear inch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,

    /// The ratio of physical pixels to device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<f64>,

    /// Support for JavaScript.
    /// `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<i32>,

    /// Indicates if the geolocation API will be available to JavaScript code
    /// running in the banner.
    /// `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geofetch: Option<i32>,

    /// Version of Flash supported by the browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashver: Option<String>,

    /// Browser language using ISO-639-1-alpha-2 (e.g., `"en"`).
    /// Only one of `language` or `langb` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Browser language using IETF BCP 47 (e.g., `"en-US"`).
    /// Only one of `language` or `langb` should be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langb: Option<String>,

    /// Carrier or ISP (e.g., `"VERIZON"`) using exchange-curated string names
    /// which should be published to bidders a priori.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Mobile carrier as the concatenated MCC-MNC code
    /// (e.g., `"310-005"` identifies Verizon Wireless CDMA in the USA).
    ///
    /// The dash between the MCC and MNC parts is required to remove parsing
    /// ambiguity.  The MCC-MNC values represent the SIM installed on the
    /// device and do not change when a device is roaming.
    ///
    /// See: <https://en.wikipedia.org/wiki/Mobile_country_code>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccmnc: Option<String>,

    /// Network connection type.
    ///
    /// Refer to AdCOM 1.0 List: Connection Types.
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | 0 | Unknown |
    /// | 1 | Ethernet |
    /// | 2 | WiFi |
    /// | 3 | Cellular Network - Unknown Generation |
    /// | 4 | Cellular Network - 2G |
    /// | 5 | Cellular Network - 3G |
    /// | 6 | Cellular Network - 4G |
    /// | 7 | Cellular Network - 5G |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectiontype: Option<i32>,

    /// ID sanctioned for advertiser use in the clear (i.e., not hashed).
    ///
    /// Unless prior arrangements have been made between the buyer and the
    /// seller directly, the value in this field is expected to be an ID
    /// derived from a call to an advertising API provided by the device's
    /// Operating System.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifa: Option<String>,

    /// **DEPRECATED as of OpenRTB 2.6.**
    /// SHA1 hashed device ID; IMEI when available, else MEID or ESN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didsha1: Option<String>,

    /// **DEPRECATED as of OpenRTB 2.6.**
    /// MD5 hashed device ID; IMEI when available, else MEID or ESN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didmd5: Option<String>,

    /// **DEPRECATED as of OpenRTB 2.6.**
    /// SHA1 hashed platform-specific ID (e.g., Android ID, IDFA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpidsha1: Option<String>,

    /// **DEPRECATED as of OpenRTB 2.6.**
    /// MD5 hashed platform-specific ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpidmd5: Option<String>,

    /// **DEPRECATED as of OpenRTB 2.6.**
    /// SHA1 hashed device MAC address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macsha1: Option<String>,

    /// **DEPRECATED as of OpenRTB 2.6.**
    /// MD5 hashed device MAC address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macmd5: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_empty_roundtrip() {
        let d = Device::default();
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(json, "{}");
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_minimal_mobile_roundtrip() {
        let d = Device {
            ua: Some("Mozilla/5.0 (iPhone; CPU iPhone OS 6_1 like Mac OS X)".to_string()),
            ifa: Some("AA000DFE74168477C70D291f574D344790E0BB11".to_string()),
            make: Some("Apple".to_string()),
            model: Some("iPhone".to_string()),
            os: Some("iOS".to_string()),
            osv: Some("6.1".to_string()),
            devicetype: Some(4),
            js: Some(1),
            connectiontype: Some(3),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"ua\""));
        assert!(json.contains("\"ifa\""));
        assert!(json.contains("\"make\":\"Apple\""));
        assert!(json.contains("\"model\":\"iPhone\""));
        assert!(json.contains("\"os\":\"iOS\""));
        assert!(json.contains("\"devicetype\":4"));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_dnt_lmt_roundtrip() {
        let d = Device {
            dnt: Some(0),
            lmt: Some(0),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"dnt\":0"));
        assert!(json.contains("\"lmt\":0"));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_ip_ipv6_roundtrip() {
        let d = Device {
            ip: Some("192.168.1.1".to_string()),
            ipv6: Some("2001:db8::1".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"ip\":\"192.168.1.1\""));
        assert!(json.contains("\"ipv6\":\"2001:db8::1\""));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_screen_dimensions_roundtrip() {
        let d = Device {
            h: Some(2532),
            w: Some(1170),
            ppi: Some(460),
            pxratio: Some(3.0),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"h\":2532"));
        assert!(json.contains("\"w\":1170"));
        assert!(json.contains("\"ppi\":460"));
        assert!(json.contains("\"pxratio\":3.0"));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_language_roundtrip() {
        let d = Device {
            language: Some("en".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"language\":\"en\""));
        assert!(!json.contains("\"langb\""));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_langb_roundtrip() {
        let d = Device {
            langb: Some("en-US".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"langb\":\"en-US\""));
        assert!(!json.contains("\"language\""));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_carrier_mccmnc_roundtrip() {
        let d = Device {
            carrier: Some("VERIZON".to_string()),
            mccmnc: Some("310-005".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"carrier\":\"VERIZON\""));
        assert!(json.contains("\"mccmnc\":\"310-005\""));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_with_geo_roundtrip() {
        let d = Device {
            geo: Some(Geo {
                lat: Some(37.7749),
                lon: Some(-122.4194),
                geo_type: Some(1),
                country: Some("USA".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"geo\""));
        assert!(json.contains("\"lat\":37.7749"));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_with_sua_roundtrip() {
        use super::super::brand_version::BrandVersion;

        let d = Device {
            ua: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string()),
            sua: Some(UserAgent {
                browsers: Some(vec![BrandVersion {
                    brand: "Google Chrome".to_string(),
                    version: Some(vec!["114".to_string()]),
                    ext: None,
                }]),
                platform: Some(BrandVersion {
                    brand: "Windows".to_string(),
                    version: Some(vec!["10".to_string(), "0".to_string()]),
                    ext: None,
                }),
                mobile: Some(0),
                architecture: Some("x86".to_string()),
                bitness: Some("64".to_string()),
                source: 1,
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"sua\""));
        assert!(json.contains("\"browsers\""));
        assert!(json.contains("\"platform\""));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_deprecated_fields_still_serialise() {
        // Deprecated fields are retained for backwards compatibility.
        let d = Device {
            didsha1: Some("dep-sha1-did".to_string()),
            didmd5: Some("dep-md5-did".to_string()),
            dpidsha1: Some("dep-sha1-dpid".to_string()),
            dpidmd5: Some("dep-md5-dpid".to_string()),
            macsha1: Some("dep-sha1-mac".to_string()),
            macmd5: Some("dep-md5-mac".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"didsha1\""));
        assert!(json.contains("\"didmd5\""));
        assert!(json.contains("\"dpidsha1\""));
        assert!(json.contains("\"dpidmd5\""));
        assert!(json.contains("\"macsha1\""));
        assert!(json.contains("\"macmd5\""));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    /// Replicates the device object from spec Example 3 (Mobile).
    #[test]
    fn spec_example_3_mobile_device() {
        let raw = r#"{
            "dnt": 0,
            "ua": "Mozilla/5.0 (iPhone; CPU iPhone OS 6_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9A334 Safari/7534.48.3",
            "ifa": "AA000DFE74168477C70D291f574D344790E0BB11",
            "carrier": "VERIZON",
            "language": "en",
            "make": "Apple",
            "model": "iPhone",
            "os": "iOS",
            "osv": "6.1",
            "js": 1,
            "connectiontype": 3,
            "devicetype": 1
        }"#;
        let d: Device = serde_json::from_str(raw).unwrap();
        assert_eq!(d.dnt, Some(0));
        assert_eq!(d.make.as_deref(), Some("Apple"));
        assert_eq!(d.model.as_deref(), Some("iPhone"));
        assert_eq!(d.os.as_deref(), Some("iOS"));
        assert_eq!(d.osv.as_deref(), Some("6.1"));
        assert_eq!(d.carrier.as_deref(), Some("VERIZON"));
        assert_eq!(d.language.as_deref(), Some("en"));
        assert_eq!(d.js, Some(1));
        assert_eq!(d.connectiontype, Some(3));
        assert_eq!(d.devicetype, Some(1));
    }

    /// Replicates the device object from spec Example 4 (desktop with UA).
    #[test]
    fn spec_example_4_desktop_device() {
        let raw = r#"{
            "ua": "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.6; en-US; rv:1.9.2.16) Gecko/20110319 Firefox/3.6.16",
            "os": "OS X",
            "js": 1
        }"#;
        let d: Device = serde_json::from_str(raw).unwrap();
        assert!(d.ua.as_deref().unwrap().contains("Firefox"));
        assert_eq!(d.os.as_deref(), Some("OS X"));
        assert_eq!(d.js, Some(1));
    }

    #[test]
    fn device_hwv_flashver_roundtrip() {
        let d = Device {
            hwv: Some("5S".to_string()),
            flashver: Some("11.1".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"hwv\":\"5S\""));
        assert!(json.contains("\"flashver\":\"11.1\""));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_geofetch_roundtrip() {
        let d = Device {
            geofetch: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"geofetch\":1"));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }

    #[test]
    fn device_with_ext_roundtrip() {
        let d = Device {
            make: Some("Samsung".to_string()),
            ext: Some(serde_json::json!({ "custom_signal": "abc" })),
            ..Default::default()
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"ext\":{\"custom_signal\":\"abc\"}"));
        let decoded: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }
}
