//! Geo object — Section 3.2.19
//!
//! This object encapsulates various methods for specifying a geographic
//! location. When subordinate to a [`Device`](super::device::Device) object,
//! it indicates the location of the device which can also be interpreted as
//! the user's current location. When subordinate to a
//! [`User`](super::user::User) object, it indicates the location of the user's
//! home base (i.e., not necessarily their current location).
//!
//! The `lat`/`lon` attributes should only be passed if they conform to the
//! accuracy depicted in the `type` attribute. For example, the centroid of a
//! geographic region such as postal code should not be passed.

use serde::{Deserialize, Serialize};

/// Geographic location — Section 3.2.19
///
/// # Location type guidance
///
/// | `type` value | Meaning |
/// |---|---|
/// | 1 | GPS / Location Services |
/// | 2 | IP Address |
/// | 3 | User Provided (e.g., registration data) |
///
/// When `type = 1` (GPS), the `accuracy` field should also be populated with
/// the value reported by the device's OS.
///
/// # Example
/// ```rust
/// use openrtb26::Geo;
///
/// let geo = Geo {
///     lat: Some(37.7749),
///     lon: Some(-122.4194),
///     geo_type: Some(1),
///     country: Some("USA".to_string()),
///     city: Some("SFO".to_string()),
///     zip: Some("94105".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Geo {
    /// Latitude from -90.0 to +90.0, where negative is south.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    /// Longitude from -180.0 to +180.0, where negative is west.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,

    /// Source of location data; recommended when passing `lat`/`lon`.
    ///
    /// Refer to AdCOM 1.0 List: Location Types.
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | 1 | GPS / Location Services |
    /// | 2 | IP Address |
    /// | 3 | User Provided |
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub geo_type: Option<i32>,

    /// Estimated location accuracy in metres; recommended when `lat`/`lon`
    /// are specified and derived from a device's location services
    /// (i.e., `type = 1`).
    ///
    /// Note that this is the accuracy as reported from the device. Consult
    /// OS-specific documentation (e.g., Android, iOS) for exact
    /// interpretation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<i32>,

    /// Number of seconds since this geolocation fix was established. Note that
    /// devices may cache location data across multiple fetches. Ideally, this
    /// value should be from the time the actual fix was taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastfix: Option<i32>,

    /// Service or provider used to determine geolocation from IP address if
    /// applicable (i.e., `type = 2`).
    ///
    /// Refer to AdCOM 1.0 List: IP Location Services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipservice: Option<i32>,

    /// Country code using ISO-3166-1-alpha-3 (e.g., `"USA"`, `"GBR"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Region code using ISO-3166-2; 2-letter state code if USA
    /// (e.g., `"CA"` for California).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Region of a country using FIPS 10-4 notation. While OpenRTB supports
    /// this attribute, it was withdrawn by NIST in 2008.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regionfips104: Option<String>,

    /// Google metro code; similar to but not exactly Nielsen DMAs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metro: Option<String>,

    /// City using United Nations Code for Trade & Transport Locations
    /// (e.g., `"SFO"` for San Francisco).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    /// Local time as the number +/- of minutes from UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utcoffset: Option<i32>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geo_empty_roundtrip() {
        let g = Geo::default();
        let json = serde_json::to_string(&g).unwrap();
        assert_eq!(json, "{}");
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_lat_lon_type_roundtrip() {
        let g = Geo {
            lat: Some(37.7749),
            lon: Some(-122.4194),
            geo_type: Some(1),
            accuracy: Some(50),
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("\"lat\":37.7749"));
        assert!(json.contains("\"lon\":-122.4194"));
        // Must be serialised as "type", not "geo_type"
        assert!(json.contains("\"type\":1"));
        assert!(!json.contains("\"geo_type\""));
        assert!(json.contains("\"accuracy\":50"));
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_type_field_name_is_type_not_geo_type() {
        // Verify JSON round-trip through the rename
        let raw = r#"{"type":2,"country":"USA"}"#;
        let g: Geo = serde_json::from_str(raw).unwrap();
        assert_eq!(g.geo_type, Some(2));
        assert_eq!(g.country.as_deref(), Some("USA"));
        let re_serialised = serde_json::to_string(&g).unwrap();
        assert!(re_serialised.contains("\"type\":2"));
        assert!(!re_serialised.contains("geo_type"));
    }

    #[test]
    fn geo_country_region_city_zip_roundtrip() {
        let g = Geo {
            country: Some("USA".to_string()),
            region: Some("CA".to_string()),
            city: Some("SFO".to_string()),
            zip: Some("94105".to_string()),
            metro: Some("807".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("\"country\":\"USA\""));
        assert!(json.contains("\"region\":\"CA\""));
        assert!(json.contains("\"city\":\"SFO\""));
        assert!(json.contains("\"zip\":\"94105\""));
        assert!(json.contains("\"metro\":\"807\""));
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_ip_based_roundtrip() {
        let g = Geo {
            geo_type: Some(2),
            ipservice: Some(3),
            country: Some("GBR".to_string()),
            region: Some("ENG".to_string()),
            city: Some("LON".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("\"type\":2"));
        assert!(json.contains("\"ipservice\":3"));
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_utcoffset_roundtrip() {
        let g = Geo {
            country: Some("USA".to_string()),
            utcoffset: Some(-300), // UTC-5
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("\"utcoffset\":-300"));
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_lastfix_roundtrip() {
        let g = Geo {
            lat: Some(51.5074),
            lon: Some(-0.1278),
            geo_type: Some(1),
            accuracy: Some(100),
            lastfix: Some(30),
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("\"lastfix\":30"));
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_regionfips104_roundtrip() {
        let g = Geo {
            country: Some("USA".to_string()),
            regionfips104: Some("US-CA".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("\"regionfips104\":\"US-CA\""));
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_with_ext_roundtrip() {
        let g = Geo {
            country: Some("DEU".to_string()),
            ext: Some(serde_json::json!({ "custom_field": "value" })),
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("\"ext\":{\"custom_field\":\"value\"}"));
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_full_roundtrip() {
        let g = Geo {
            lat: Some(48.8566),
            lon: Some(2.3522),
            geo_type: Some(1),
            accuracy: Some(20),
            lastfix: Some(10),
            ipservice: None,
            country: Some("FRA".to_string()),
            region: Some("IDF".to_string()),
            regionfips104: None,
            metro: Some("75001".to_string()),
            city: Some("PAR".to_string()),
            zip: Some("75001".to_string()),
            utcoffset: Some(60),
            ext: None,
        };
        let json = serde_json::to_string(&g).unwrap();
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }

    #[test]
    fn geo_negative_lat_lon() {
        let g = Geo {
            lat: Some(-33.8688),
            lon: Some(151.2093),
            geo_type: Some(1),
            country: Some("AUS".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("\"lat\":-33.8688"));
        assert!(json.contains("\"lon\":151.2093"));
        let decoded: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(g, decoded);
    }
}
