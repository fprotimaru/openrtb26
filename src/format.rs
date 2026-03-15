use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Format {
    /// Width in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    /// Height in device independent pixels (DIPS)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    /// Relative width when expressing size as a ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,
    /// Relative height when expressing size as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,
    /// The minimum width in device independent pixels (DIPS) at
    // which the ad will be displayed the size is expressed as a
    // ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,
    /// Placeholder for exchange-specific extensions to OpenRTB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- Serialization ---

    #[test]
    fn empty_format_serializes_to_empty_object() {
        let format = Format::default();
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#"{}"#);
    }

    #[test]
    fn format_with_fixed_size() {
        let format = Format {
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#"{"w":300,"h":250}"#);
    }

    #[test]
    fn format_with_ratio() {
        let format = Format {
            wratio: Some(16),
            hratio: Some(9),
            wmin: Some(320),
            ..Default::default()
        };
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#"{"wratio":16,"hratio":9,"wmin":320}"#);
    }

    #[test]
    fn format_with_ext() {
        let format = Format {
            ext: Some(json!({"custom_key": 42})),
            ..Default::default()
        };
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#"{"ext":{"custom_key":42}}"#);
    }

    #[test]
    fn none_fields_are_skipped_in_serialization() {
        let format = Format {
            w: Some(100),
            ..Default::default()
        };
        let json = serde_json::to_string(&format).unwrap();
        // h, wratio, hratio, wmin, ext must not appear
        assert!(!json.contains("\"h\""));
        assert!(!json.contains("\"wratio\""));
        assert!(!json.contains("\"hratio\""));
        assert!(!json.contains("\"wmin\""));
        assert!(!json.contains("\"ext\""));
    }

    // --- Deserialization ---

    #[test]
    fn empty_object_deserializes_to_default() {
        let format: Format = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(format.w, None);
        assert_eq!(format.h, None);
        assert_eq!(format.wratio, None);
        assert_eq!(format.hratio, None);
        assert_eq!(format.wmin, None);
        assert_eq!(format.ext, None);
    }

    #[test]
    fn format_deserializes_fixed_size() {
        let format: Format = serde_json::from_str(r#"{"w":300,"h":250}"#).unwrap();
        assert_eq!(format.w, Some(300));
        assert_eq!(format.h, Some(250));
        assert_eq!(format.wratio, None);
    }

    #[test]
    fn format_deserializes_ratio() {
        let format: Format =
            serde_json::from_str(r#"{"wratio":16,"hratio":9,"wmin":320}"#).unwrap();
        assert_eq!(format.wratio, Some(16));
        assert_eq!(format.hratio, Some(9));
        assert_eq!(format.wmin, Some(320));
    }

    #[test]
    fn format_deserializes_ext() {
        let format: Format = serde_json::from_str(r#"{"ext":{"foo":"bar"}}"#).unwrap();
        assert_eq!(format.ext, Some(json!({"foo": "bar"})));
    }

    #[test]
    fn unknown_fields_are_ignored() {
        // OpenRTB consumers should be tolerant of unknown fields
        let result: Result<Format, _> = serde_json::from_str(r#"{"w":100,"unknown_field":999}"#);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().w, Some(100));
    }

    // --- Roundtrip ---

    #[test]
    fn roundtrip_fixed_size() {
        let original = Format {
            w: Some(320),
            h: Some(50),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Format = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.w, original.w);
        assert_eq!(restored.h, original.h);
    }

    #[test]
    fn roundtrip_full() {
        let original = Format {
            w: Some(300),
            h: Some(250),
            wratio: Some(4),
            hratio: Some(3),
            wmin: Some(100),
            ext: Some(json!({"key": "value"})),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Format = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.w, original.w);
        assert_eq!(restored.h, original.h);
        assert_eq!(restored.wratio, original.wratio);
        assert_eq!(restored.hratio, original.hratio);
        assert_eq!(restored.wmin, original.wmin);
        assert_eq!(restored.ext, original.ext);
    }
}
