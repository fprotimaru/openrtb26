use super::format::Format;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Banner {
    /// Array of format objects (Section 3.2.10) representing the banner sizes
    /// permitted. If none are specified, then use of the h and w attributes is
    /// highly recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    /// Exact width in device-independent pixels (DIPS); recommended if no
    /// format objects are specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    /// Exact height in device-independent pixels (DIPS); recommended if no
    /// format objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    /// Blocked banner ad types.
    ///
    /// | Value | Meaning         |
    /// |-------|-----------------|
    /// | 1     | XHTML Text Ad   |
    /// | 2     | XHTML Banner Ad |
    /// | 3     | JavaScript Ad   |
    /// | 4     | iframe          |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topframe: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expdir: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcm: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- Serialization ---

    #[test]
    fn empty_banner_serializes_to_empty_object() {
        let banner = Banner::default();
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, r#"{}"#);
    }

    #[test]
    fn banner_with_basic_dimensions() {
        let banner = Banner {
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, r#"{"w":300,"h":250}"#);
    }

    #[test]
    fn banner_with_format() {
        let banner = Banner {
            format: Some(Format {
                w: Some(728),
                h: Some(90),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, r#"{"format":{"w":728,"h":90}}"#);
    }

    #[test]
    fn banner_with_blocked_types() {
        let banner = Banner {
            btype: Some(vec![1, 4]),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, r#"{"btype":[1,4]}"#);
    }

    #[test]
    fn banner_with_attributes() {
        let banner = Banner {
            battr: Some(vec![1, 2, 3]),
            pos: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, r#"{"battr":[1,2,3],"pos":1}"#);
    }

    #[test]
    fn banner_with_mimes() {
        let banner = Banner {
            mimes: Some(vec!["image/jpeg".to_string(), "image/png".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, r#"{"mimes":["image/jpeg","image/png"]}"#);
    }

    #[test]
    fn banner_with_api_and_expdir() {
        let banner = Banner {
            api: Some(vec![1, 2]),
            expdir: Some(vec![2, 3]),
            topframe: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["api"], json!([1, 2]));
        assert_eq!(parsed["expdir"], json!([2, 3]));
        assert_eq!(parsed["topframe"], json!(1));
        assert_eq!(parsed.as_object().unwrap().len(), 3);
    }

    #[test]
    fn banner_with_id_and_vcm() {
        let banner = Banner {
            id: Some("banner123".to_string()),
            vcm: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, r#"{"id":"banner123","vcm":1}"#);
    }

    #[test]
    fn banner_with_ext() {
        let banner = Banner {
            ext: Some(json!({"custom": "value", "numeric": 42})),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        assert_eq!(json, r#"{"ext":{"custom":"value","numeric":42}}"#);
    }

    #[test]
    fn none_fields_are_skipped_in_serialization() {
        let banner = Banner {
            w: Some(100),
            h: Some(50),
            ..Default::default()
        };
        let json = serde_json::to_string(&banner).unwrap();
        // Other fields must not appear
        assert!(!json.contains("\"format\""));
        assert!(!json.contains("\"btype\""));
        assert!(!json.contains("\"battr\""));
        assert!(!json.contains("\"pos\""));
        assert!(!json.contains("\"mimes\""));
        assert!(!json.contains("\"topframe\""));
        assert!(!json.contains("\"expdir\""));
        assert!(!json.contains("\"api\""));
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"vcm\""));
        assert!(!json.contains("\"ext\""));
    }

    // --- Deserialization ---

    #[test]
    fn empty_object_deserializes_to_default() {
        let banner: Banner = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(banner.w, None);
        assert_eq!(banner.h, None);
        assert_eq!(banner.format, None);
        assert_eq!(banner.btype, None);
        assert_eq!(banner.battr, None);
        assert_eq!(banner.pos, None);
        assert_eq!(banner.mimes, None);
        assert_eq!(banner.topframe, None);
        assert_eq!(banner.expdir, None);
        assert_eq!(banner.api, None);
        assert_eq!(banner.id, None);
        assert_eq!(banner.vcm, None);
        assert_eq!(banner.ext, None);
    }

    #[test]
    fn banner_deserializes_basic_dimensions() {
        let banner: Banner = serde_json::from_str(r#"{"w":300,"h":250}"#).unwrap();
        assert_eq!(banner.w, Some(300));
        assert_eq!(banner.h, Some(250));
        assert_eq!(banner.format, None);
    }

    #[test]
    fn banner_deserializes_format() {
        let banner: Banner = serde_json::from_str(r#"{"format":{"w":728,"h":90}}"#).unwrap();
        assert!(banner.format.is_some());
        let format = banner.format.unwrap();
        assert_eq!(format.w, Some(728));
        assert_eq!(format.h, Some(90));
    }

    #[test]
    fn banner_deserializes_blocked_types() {
        let banner: Banner = serde_json::from_str(r#"{"btype":[1,2,4]}"#).unwrap();
        assert_eq!(banner.btype, Some(vec![1, 2, 4]));
    }

    #[test]
    fn banner_deserializes_attributes_and_position() {
        let banner: Banner = serde_json::from_str(r#"{"battr":[3,5],"pos":2}"#).unwrap();
        assert_eq!(banner.battr, Some(vec![3, 5]));
        assert_eq!(banner.pos, Some(2));
    }

    #[test]
    fn banner_deserializes_mimes() {
        let banner: Banner =
            serde_json::from_str(r#"{"mimes":["video/mp4","video/webm"]}"#).unwrap();
        assert_eq!(
            banner.mimes,
            Some(vec!["video/mp4".to_string(), "video/webm".to_string()])
        );
    }

    #[test]
    fn banner_deserializes_complex_fields() {
        let banner: Banner =
            serde_json::from_str(r#"{"api":[3],"expdir":[1,2],"topframe":0,"id":"test","vcm":1}"#)
                .unwrap();
        assert_eq!(banner.api, Some(vec![3]));
        assert_eq!(banner.expdir, Some(vec![1, 2]));
        assert_eq!(banner.topframe, Some(0));
        assert_eq!(banner.id, Some("test".to_string()));
        assert_eq!(banner.vcm, Some(1));
    }

    #[test]
    fn banner_deserializes_ext() {
        let banner: Banner = serde_json::from_str(r#"{"ext":{"foo":"bar","num":123}}"#).unwrap();
        assert_eq!(banner.ext, Some(json!({"foo": "bar", "num": 123})));
    }

    #[test]
    fn unknown_fields_are_ignored() {
        // OpenRTB consumers should be tolerant of unknown fields
        let result: Result<Banner, _> =
            serde_json::from_str(r#"{"w":100,"unknown_field":"value","h":50}"#);
        assert!(result.is_ok());
        let banner = result.unwrap();
        assert_eq!(banner.w, Some(100));
        assert_eq!(banner.h, Some(50));
    }

    #[test]
    fn deserialization_with_empty_arrays() {
        let banner: Banner = serde_json::from_str(r#"{"btype":[],"mimes":[]}"#).unwrap();
        assert_eq!(banner.btype, Some(vec![]));
        assert_eq!(banner.mimes, Some(vec![]));
    }

    // --- Roundtrip ---

    #[test]
    fn roundtrip_basic_dimensions() {
        let original = Banner {
            w: Some(320),
            h: Some(50),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Banner = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.w, original.w);
        assert_eq!(restored.h, original.h);
    }

    #[test]
    fn roundtrip_with_format() {
        let original = Banner {
            format: Some(Format {
                w: Some(300),
                h: Some(250),
                wratio: Some(16),
                hratio: Some(9),
                wmin: Some(320),
                ext: Some(json!({"ratio_based": true})),
            }),
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Banner = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.w, original.w);
        assert_eq!(restored.h, original.h);
        assert!(restored.format.is_some());
        let original_format = original.format.unwrap();
        let restored_format = restored.format.unwrap();
        assert_eq!(restored_format.w, original_format.w);
        assert_eq!(restored_format.h, original_format.h);
        assert_eq!(restored_format.wratio, original_format.wratio);
        assert_eq!(restored_format.hratio, original_format.hratio);
        assert_eq!(restored_format.wmin, original_format.wmin);
        assert_eq!(restored_format.ext, original_format.ext);
    }

    #[test]
    fn roundtrip_full() {
        let original = Banner {
            format: Some(Format {
                w: Some(728),
                h: Some(90),
                ..Default::default()
            }),
            w: Some(728),
            h: Some(90),
            btype: Some(vec![1, 3]),
            battr: Some(vec![1, 2, 3]),
            pos: Some(3),
            mimes: Some(vec!["image/jpeg".to_string(), "image/png".to_string()]),
            topframe: Some(1),
            expdir: Some(vec![2]),
            api: Some(vec![1, 2, 3]),
            id: Some("banner_728x90".to_string()),
            vcm: Some(0),
            ext: Some(json!({"preferred_size": "leaderboard"})),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Banner = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.w, original.w);
        assert_eq!(restored.h, original.h);
        assert_eq!(restored.btype, original.btype);
        assert_eq!(restored.battr, original.battr);
        assert_eq!(restored.pos, original.pos);
        assert_eq!(restored.mimes, original.mimes);
        assert_eq!(restored.topframe, original.topframe);
        assert_eq!(restored.expdir, original.expdir);
        assert_eq!(restored.api, original.api);
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.vcm, original.vcm);
        assert_eq!(restored.ext, original.ext);

        // Compare format
        assert!(restored.format.is_some());
        let original_format = original.format.unwrap();
        let restored_format = restored.format.unwrap();
        assert_eq!(restored_format.w, original_format.w);
        assert_eq!(restored_format.h, original_format.h);
    }
}
