//! Site, App, and DOOH objects — Sections 3.2.13, 3.2.14, and 3.2.32
//!
//! - [`Site`] — Section 3.2.13 — Details of the website calling for the impression.
//! - [`App`]  — Section 3.2.14 — Details of the application calling for the impression.
//! - [`Dooh`] — Section 3.2.32 — Details of the Digital Out-Of-Home inventory.
//!
//! A bid request must not contain more than one of a `Site`, `App`, or `DOOH`
//! object.

use serde::{Deserialize, Serialize};

use super::{content::Content, segment::Publisher};

// ── Site ──────────────────────────────────────────────────────────────────────

/// Details of the website calling for the impression — Section 3.2.13
///
/// This object should be included if the ad supported content is a website as
/// opposed to a non-browser application or Digital Out of Home (DOOH)
/// inventory. A bid request must not contain more than one of a `Site`, `App`,
/// or `DOOH` object. At a minimum, it is useful to provide a site ID or page
/// URL, but this is not strictly required.
///
/// # Example
/// ```rust
/// use openrtb26::{Site, Publisher};
///
/// let site = Site {
///     id: Some("102855".to_string()),
///     domain: Some("www.foobar.com".to_string()),
///     page: Some("http://www.foobar.com/1234.html".to_string()),
///     cat: Some(vec!["IAB3-1".to_string()]),
///     publisher: Some(Publisher {
///         id: Some("8953".to_string()),
///         name: Some("foobar.com".to_string()),
///         domain: Some("foobar.com".to_string()),
///         ..Default::default()
///     }),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Site {
    /// Exchange-specific site ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Site name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Domain of the site (e.g., `"mysite.foo.com"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// The taxonomy in use for `cat`, `sectioncat`, and `pagecat`.
    ///
    /// Refer to AdCOM 1.0 List: Category Taxonomies.
    /// Default `1` = IAB Content Category Taxonomy 1.0.
    #[serde(default = "default_cattax", skip_serializing_if = "is_default_cattax")]
    pub cattax: i32,

    /// Array of IAB Tech Lab content categories of the site.
    /// The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Array of IAB Tech Lab content categories that describe the current
    /// section of the site.
    /// The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Array of IAB Tech Lab content categories that describe the current
    /// page or view of the site.
    /// The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// URL of the page where the impression will be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Referrer URL that caused navigation to the current page.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub page_ref: Option<String>,

    /// Search string that caused navigation to the current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Indicates if the site has been programmed to optimize layout when
    /// viewed on mobile devices.
    /// `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i32>,

    /// Indicates if the site has a privacy policy.
    /// `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,

    /// Details about the [`Publisher`] of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    /// Details about the [`Content`] within the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Comma-separated list of keywords about the site.
    /// Only one of `keywords` or `kwarray` may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Array of keywords about the site.
    /// Only one of `keywords` or `kwarray` may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwarray: Option<Vec<String>>,

    /// A domain to be used for inventory authorization in the case of
    /// inventory sharing arrangements between a site owner and content owner.
    ///
    /// This field is typically used by authorization crawlers to establish the
    /// domain of the content owner, who has the right to monetize some portion
    /// of ad inventory within the site. The content owner's domain should be
    /// listed in the site owner's `ads.txt` file as an
    /// `inventorypartnerdomain`. Authorization for supply from the
    /// `inventorypartnerdomain` will be published in the `ads.txt` file on the
    /// root of that domain.
    ///
    /// Refer to [the ads.txt 1.1 spec](https://iabtechlab.com/wp-content/uploads/2022/04/Ads.txt-1.1.pdf)
    /// for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventorypartnerdomain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── App ───────────────────────────────────────────────────────────────────────

/// Details of the application calling for the impression — Section 3.2.14
///
/// This object should be included if the ad supported content is a non-browser
/// application (typically in mobile) as opposed to a website. A bid request
/// must not contain more than one of a `Site`, `App`, or `DOOH` object. At a
/// minimum, it is useful to provide an App ID or bundle, but this is not
/// strictly required.
///
/// # Example
/// ```rust
/// use openrtb26::{App, Publisher};
///
/// let app = App {
///     id: Some("agltb3B1Yi1pbmNyDAsSA0FwcBiJkfIUDA".to_string()),
///     name: Some("Yahoo Weather".to_string()),
///     bundle: Some("12345".to_string()),
///     cat: Some(vec!["IAB15".to_string(), "IAB15-10".to_string()]),
///     ver: Some("1.0.2".to_string()),
///     storeurl: Some("https://itunes.apple.com/id628677149".to_string()),
///     publisher: Some(Publisher {
///         id: Some("agltb3B1Yi1pbmNyDAsSA0FwcBiJkfTUCV".to_string()),
///         name: Some("yahoo".to_string()),
///         domain: Some("www.yahoo.com".to_string()),
///         ..Default::default()
///     }),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct App {
    /// Exchange-specific app ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// App name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The store ID of the app in an app store.
    ///
    /// See [OTT/CTV Store Assigned App Identification Guidelines](https://iabtechlab.com/wp-content/uploads/2020/08/IAB-Tech-Lab-OTT-store-assigned-App-Identification-Guidelines-2020.pdf)
    /// for more details about expected strings for CTV app stores.
    ///
    /// For mobile apps in Google Play Store, these should be bundle or package
    /// names (e.g. `"com.foo.mygame"`). For apps in Apple App Store, these
    /// should be a numeric ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    /// Domain of the app (e.g., `"mygame.foo.com"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// App store URL for an installed app; for IQG 2.1 compliance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,

    /// The taxonomy in use for `cat`, `sectioncat`, and `pagecat`.
    ///
    /// Refer to AdCOM 1.0 List: Category Taxonomies.
    /// Default `1` = IAB Content Category Taxonomy 1.0.
    #[serde(default = "default_cattax", skip_serializing_if = "is_default_cattax")]
    pub cattax: i32,

    /// Array of IAB Tech Lab content categories of the app.
    /// The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Array of IAB Tech Lab content categories that describe the current
    /// section of the app.
    /// The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Array of IAB Tech Lab content categories that describe the current
    /// page or view of the app.
    /// The taxonomy to be used is defined by the `cattax` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// Application version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// Indicates if the app has a privacy policy.
    /// `0` = no, `1` = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,

    /// `0` = app is free, `1` = the app is a paid version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i32>,

    /// Details about the [`Publisher`] of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    /// Details about the [`Content`] within the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Comma-separated list of keywords about the app.
    /// Only one of `keywords` or `kwarray` may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Array of keywords about the app.
    /// Only one of `keywords` or `kwarray` may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwarray: Option<Vec<String>>,

    /// A domain to be used for inventory authorization in the case of
    /// inventory sharing arrangements between an app owner and content owner.
    ///
    /// This field is typically used by authorization crawlers to establish the
    /// domain of the content owner, who has the right to monetize some portion
    /// of ad inventory within the app. The content owner's domain should be
    /// listed in the app owner's `app-ads.txt` file as an
    /// `inventorypartnerdomain`.
    ///
    /// Refer to [the ads.txt 1.1 spec](https://iabtechlab.com/wp-content/uploads/2022/04/Ads.txt-1.1.pdf)
    /// for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventorypartnerdomain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── Dooh ──────────────────────────────────────────────────────────────────────

/// Digital Out-Of-Home inventory — Section 3.2.32
///
/// This object should be included if the ad supported content is a Digital
/// Out-Of-Home screen. A bid request with a `DOOH` object must not contain a
/// `site` or `app` object. At a minimum, it is useful to provide `id` and/or
/// `venuetypeid`, but this is not strictly required.
///
/// # Example
/// ```rust
/// use openrtb26::Dooh;
///
/// let dooh = Dooh {
///     id: Some("dooh-placement-001".to_string()),
///     name: Some("Times Square Billboard".to_string()),
///     venuetype: Some(vec!["airport".to_string()]),
///     venuetypetax: 1,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Dooh {
    /// Exchange-provided ID for a placement or logical grouping of placements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the DOOH placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type(s) of out-of-home venue.
    ///
    /// The taxonomy to be used is defined by the `venuetypetax` field. If no
    /// `venuetypetax` field is supplied, the OpenOOH Venue Taxonomy is
    /// assumed.
    ///
    /// See: <https://github.com/openooh/venue-taxonomy/blob/main/specification-1.0.md>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venuetype: Option<Vec<String>>,

    /// The venue taxonomy in use.
    ///
    /// Refer to AdCOM 1.0 List: DOOH Venue Taxonomies.
    /// Default `1` = OpenOOH Venue Taxonomy.
    #[serde(
        default = "default_venuetypetax",
        skip_serializing_if = "is_default_venuetypetax"
    )]
    pub venuetypetax: i32,

    /// Details about the [`Publisher`] of the DOOH placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    /// Domain of the inventory owner (e.g., `"mysite.foo.com"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Comma-separated list of keywords about the DOOH placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Details about the [`Content`] within the DOOH placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── serde helpers ─────────────────────────────────────────────────────────────

fn default_cattax() -> i32 {
    1
}

fn is_default_cattax(v: &i32) -> bool {
    *v == 1
}

fn default_venuetypetax() -> i32 {
    1
}

fn is_default_venuetypetax(v: &i32) -> bool {
    *v == 1
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Site ──────────────────────────────────────────────────────────────────

    #[test]
    fn site_empty_roundtrip() {
        let s = Site::default();
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "{}");
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn site_minimal_roundtrip() {
        let s = Site {
            id: Some("102855".to_string()),
            domain: Some("www.foobar.com".to_string()),
            page: Some("http://www.foobar.com/1234.html".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"id\":\"102855\""));
        assert!(json.contains("\"domain\":\"www.foobar.com\""));
        assert!(json.contains("\"page\":\"http://www.foobar.com/1234.html\""));
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn site_page_ref_serialised_as_ref() {
        let s = Site {
            id: Some("s-1".to_string()),
            page_ref: Some("http://referringsite.com/referringpage.htm".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        // Must be serialised as "ref", not "page_ref"
        assert!(json.contains("\"ref\":\"http://referringsite.com/referringpage.htm\""));
        assert!(!json.contains("\"page_ref\""));
        // And deserialise back from "ref" correctly
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn site_ref_field_name_round_trips_from_raw_json() {
        let raw = r#"{"id":"s1","ref":"http://referrer.example.com/"}"#;
        let s: Site = serde_json::from_str(raw).unwrap();
        assert_eq!(s.page_ref.as_deref(), Some("http://referrer.example.com/"));
        let re_serialised = serde_json::to_string(&s).unwrap();
        assert!(re_serialised.contains("\"ref\""));
        assert!(!re_serialised.contains("page_ref"));
    }

    #[test]
    fn site_default_cattax_not_serialised() {
        let s = Site {
            cat: Some(vec!["IAB3-1".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(!json.contains("\"cattax\""));
    }

    #[test]
    fn site_non_default_cattax_serialised() {
        let s = Site {
            cattax: 3,
            cat: Some(vec!["1".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"cattax\":3"));
    }

    #[test]
    fn site_with_publisher_roundtrip() {
        let s = Site {
            id: Some("102855".to_string()),
            cat: Some(vec!["IAB3-1".to_string()]),
            domain: Some("www.foobar.com".to_string()),
            page: Some("http://www.foobar.com/1234.html".to_string()),
            publisher: Some(Publisher {
                id: Some("8953".to_string()),
                name: Some("foobar.com".to_string()),
                cat: Some(vec!["IAB3-1".to_string()]),
                domain: Some("foobar.com".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"publisher\""));
        assert!(json.contains("\"8953\""));
        assert!(json.contains("\"foobar.com\""));
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn site_with_content_roundtrip() {
        let s = Site {
            id: Some("1345135123".to_string()),
            name: Some("Site ABCD".to_string()),
            domain: Some("siteabcd.com".to_string()),
            cat: Some(vec!["IAB2-1".to_string(), "IAB2-2".to_string()]),
            page: Some("http://siteabcd.com/page.htm".to_string()),
            page_ref: Some("http://referringsite.com/referringpage.htm".to_string()),
            privacypolicy: Some(1),
            publisher: Some(Publisher {
                id: Some("pub12345".to_string()),
                name: Some("Publisher A".to_string()),
                ..Default::default()
            }),
            content: Some(Content {
                id: Some("1234567".to_string()),
                series: Some("All About Cars".to_string()),
                season: Some("2".to_string()),
                episode: Some(23),
                title: Some("Car Show".to_string()),
                cat: Some(vec!["IAB2-2".to_string()]),
                keywords: Some("keyword-a,keyword-b,keyword-c".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"content\""));
        assert!(json.contains("\"All About Cars\""));
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn site_section_page_cat_roundtrip() {
        let s = Site {
            id: Some("s-1".to_string()),
            cat: Some(vec!["IAB1".to_string()]),
            sectioncat: Some(vec!["IAB1-1".to_string()]),
            pagecat: Some(vec!["IAB1-1-1".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"sectioncat\":[\"IAB1-1\"]"));
        assert!(json.contains("\"pagecat\":[\"IAB1-1-1\"]"));
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn site_search_mobile_roundtrip() {
        let s = Site {
            search: Some("cars for sale".to_string()),
            mobile: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"search\":\"cars for sale\""));
        assert!(json.contains("\"mobile\":1"));
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn site_inventorypartnerdomain_roundtrip() {
        let s = Site {
            id: Some("s-1".to_string()),
            inventorypartnerdomain: Some("content-owner.com".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"inventorypartnerdomain\":\"content-owner.com\""));
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn site_kwarray_roundtrip() {
        let s = Site {
            kwarray: Some(vec!["news".to_string(), "politics".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"kwarray\":[\"news\",\"politics\"]"));
        let decoded: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(s, decoded);
    }

    /// Replicate the site object from spec Example 1 (Simple Banner).
    #[test]
    fn spec_example_1_site_from_json() {
        let raw = r#"{
            "id": "102855",
            "cat": ["IAB3-1"],
            "domain": "www.foobar.com",
            "page": "http://www.foobar.com/1234.html",
            "publisher": {
                "id": "8953",
                "name": "foobar.com",
                "cat": ["IAB3-1"],
                "domain": "foobar.com"
            }
        }"#;
        let s: Site = serde_json::from_str(raw).unwrap();
        assert_eq!(s.id.as_deref(), Some("102855"));
        assert_eq!(s.cat.as_ref().unwrap()[0], "IAB3-1");
        assert_eq!(s.domain.as_deref(), Some("www.foobar.com"));
        assert_eq!(s.page.as_deref(), Some("http://www.foobar.com/1234.html"));
        let pub_ = s.publisher.as_ref().unwrap();
        assert_eq!(pub_.id.as_deref(), Some("8953"));
        assert_eq!(pub_.name.as_deref(), Some("foobar.com"));
        assert_eq!(pub_.domain.as_deref(), Some("foobar.com"));
    }

    // ── App ───────────────────────────────────────────────────────────────────

    #[test]
    fn app_empty_roundtrip() {
        let a = App::default();
        let json = serde_json::to_string(&a).unwrap();
        assert_eq!(json, "{}");
        let decoded: App = serde_json::from_str(&json).unwrap();
        assert_eq!(a, decoded);
    }

    #[test]
    fn app_minimal_roundtrip() {
        let a = App {
            id: Some("app-001".to_string()),
            bundle: Some("com.example.myapp".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        assert!(json.contains("\"id\":\"app-001\""));
        assert!(json.contains("\"bundle\":\"com.example.myapp\""));
        let decoded: App = serde_json::from_str(&json).unwrap();
        assert_eq!(a, decoded);
    }

    #[test]
    fn app_default_cattax_not_serialised() {
        let a = App {
            cat: Some(vec!["IAB15".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        assert!(!json.contains("\"cattax\""));
    }

    #[test]
    fn app_paid_privacypolicy_roundtrip() {
        let a = App {
            paid: Some(0),
            privacypolicy: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        assert!(json.contains("\"paid\":0"));
        assert!(json.contains("\"privacypolicy\":1"));
        let decoded: App = serde_json::from_str(&json).unwrap();
        assert_eq!(a, decoded);
    }

    #[test]
    fn app_with_publisher_roundtrip() {
        let a = App {
            id: Some("agltb3B1Yi1pbmNyDAsSA0FwcBiJkfIUDA".to_string()),
            name: Some("Yahoo Weather".to_string()),
            cat: Some(vec!["IAB15".to_string(), "IAB15-10".to_string()]),
            ver: Some("1.0.2".to_string()),
            bundle: Some("12345".to_string()),
            storeurl: Some("https://itunes.apple.com/id628677149".to_string()),
            publisher: Some(Publisher {
                id: Some("agltb3B1Yi1pbmNyDAsSA0FwcBiJkfTUCV".to_string()),
                name: Some("yahoo".to_string()),
                domain: Some("www.yahoo.com".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        assert!(json.contains("\"name\":\"Yahoo Weather\""));
        assert!(json.contains("\"bundle\":\"12345\""));
        assert!(json.contains("\"storeurl\""));
        assert!(json.contains("\"publisher\""));
        let decoded: App = serde_json::from_str(&json).unwrap();
        assert_eq!(a, decoded);
    }
}
