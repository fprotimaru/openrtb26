//! SupplyChain and SupplyChainNode objects — Sections 3.2.25 and 3.2.26
//!
//! The `SupplyChain` object is composed of a set of nodes where each node
//! represents a specific entity that participates in the transacting of
//! inventory. The entire chain of nodes from beginning to end represents all
//! entities who are involved in the direct flow of payment for inventory.
//!
//! Detailed implementation examples:
//! <https://github.com/InteractiveAdvertisingBureau/openrtb/blob/master/supplychainobject.md>

use serde::{Deserialize, Serialize};

// ── SupplyChainNode ───────────────────────────────────────────────────────────

/// A single node in a supply chain — Section 3.2.26
///
/// Each node defines the identity of an entity participating in the supply
/// chain of a bid request. The `asi` and `sid` fields are required.
///
/// # Example
/// ```rust
/// use openrtb26::SupplyChainNode;
///
/// let node = SupplyChainNode {
///     asi: "exchange.com".to_string(),
///     sid: "pub-12345".to_string(),
///     hp: Some(1),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupplyChainNode {
    /// The canonical domain name of the SSP, Exchange, Header Wrapper, etc.
    /// system that bidders connect to. This may be the operational domain of
    /// the system if that is different from the parent corporate domain, to
    /// facilitate WHOIS and reverse IP lookups to establish clear ownership.
    ///
    /// This should be the same value as used to identify sellers in an
    /// `ads.txt` file if one exists.
    ///
    /// **Required.**
    pub asi: String,

    /// The identifier associated with the seller or reseller account within
    /// the advertising system. This must contain the same value used in
    /// transactions (i.e. OpenRTB bid requests) in the field specified by the
    /// SSP/exchange. Typically, in OpenRTB, this is `publisher.id`.
    ///
    /// Should be limited to 64 characters in length.
    ///
    /// **Required.**
    pub sid: String,

    /// The OpenRTB RequestId of the request as issued by this seller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,

    /// The name of the company (the legal entity) that is paid for inventory
    /// transacted under the given `sid`. This value is optional and should
    /// **not** be included if it already exists in the advertising system's
    /// `sellers.json` file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The business domain name of the entity represented by this node. This
    /// value is optional and should **not** be included if it already exists in
    /// the advertising system's `sellers.json` file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Indicates whether this node will be involved in the flow of payment for
    /// the inventory.
    ///
    /// `1` = the advertising system in the `asi` field pays the seller in the
    /// `sid` field, who is responsible for paying the previous node in the
    /// chain.
    ///
    /// `0` = this node is not involved in the flow of payment.
    ///
    /// For SupplyChain version 1.0 this property should always be `1`.
    /// Implementers should ensure that they propagate this field onwards when
    /// constructing SupplyChain objects in bid requests sent downstream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<i32>,

    /// Placeholder for advertising-system specific extensions to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── SupplyChain ───────────────────────────────────────────────────────────────

/// Chain of entities involved in the direct flow of payment — Section 3.2.25
///
/// The `SupplyChain` object is composed of a set of [`SupplyChainNode`]
/// objects.  In a **complete** supply chain the first node represents the
/// initial advertising system and seller ID involved in the transaction (i.e.
/// the owner of the site, app, or other medium).  In an **incomplete** supply
/// chain it represents the first *known* node.  The last node always
/// represents the entity sending this bid request.
///
/// # Example
/// ```rust
/// use openrtb26::{SupplyChain, SupplyChainNode};
///
/// let schain = SupplyChain {
///     complete: 1,
///     ver: "1.0".to_string(),
///     nodes: vec![
///         SupplyChainNode {
///             asi: "exchange.com".to_string(),
///             sid: "pub-12345".to_string(),
///             hp: Some(1),
///             ..Default::default()
///         },
///     ],
///     ext: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupplyChain {
    /// Flag indicating whether the chain contains all nodes involved in the
    /// transaction leading back to the owner of the site, app, or other medium.
    /// `0` = no, `1` = yes.
    ///
    /// **Required.**
    pub complete: i32,

    /// Array of [`SupplyChainNode`] objects in the order of the chain.
    ///
    /// **Required.**
    pub nodes: Vec<SupplyChainNode>,

    /// Version of the supply chain specification in use, in the format
    /// `"major.minor"` (e.g., `"1.0"`).
    ///
    /// **Required.**
    pub ver: String,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── SupplyChainNode ───────────────────────────────────────────────────────

    #[test]
    fn node_minimal_roundtrip() {
        let node = SupplyChainNode {
            asi: "exchange.com".to_string(),
            sid: "pub-12345".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"asi\":\"exchange.com\""));
        assert!(json.contains("\"sid\":\"pub-12345\""));
        let decoded: SupplyChainNode = serde_json::from_str(&json).unwrap();
        assert_eq!(node, decoded);
    }

    #[test]
    fn node_full_roundtrip() {
        let node = SupplyChainNode {
            asi: "reseller.example".to_string(),
            sid: "reseller-account-99".to_string(),
            rid: Some("request-xyz".to_string()),
            name: Some("Reseller Inc.".to_string()),
            domain: Some("reseller.example".to_string()),
            hp: Some(1),
            ext: Some(serde_json::json!({ "custom": true })),
        };
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"rid\":\"request-xyz\""));
        assert!(json.contains("\"name\":\"Reseller Inc.\""));
        assert!(json.contains("\"domain\":\"reseller.example\""));
        assert!(json.contains("\"hp\":1"));
        assert!(json.contains("\"ext\""));
        let decoded: SupplyChainNode = serde_json::from_str(&json).unwrap();
        assert_eq!(node, decoded);
    }

    #[test]
    fn node_optional_fields_absent_when_not_set() {
        let node = SupplyChainNode {
            asi: "exchange.com".to_string(),
            sid: "12345".to_string(),
            hp: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert!(!json.contains("\"rid\""));
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"domain\""));
        assert!(!json.contains("\"ext\""));
    }

    // ── SupplyChain ───────────────────────────────────────────────────────────

    #[test]
    fn supply_chain_minimal_roundtrip() {
        let sc = SupplyChain {
            complete: 1,
            nodes: vec![],
            ver: "1.0".to_string(),
            ext: None,
        };
        let json = serde_json::to_string(&sc).unwrap();
        assert!(json.contains("\"complete\":1"));
        assert!(json.contains("\"nodes\":[]"));
        assert!(json.contains("\"ver\":\"1.0\""));
        let decoded: SupplyChain = serde_json::from_str(&json).unwrap();
        assert_eq!(sc, decoded);
    }

    #[test]
    fn supply_chain_with_single_node_roundtrip() {
        let sc = SupplyChain {
            complete: 1,
            ver: "1.0".to_string(),
            nodes: vec![SupplyChainNode {
                asi: "directseller.com".to_string(),
                sid: "00001".to_string(),
                hp: Some(1),
                ..Default::default()
            }],
            ext: None,
        };
        let json = serde_json::to_string(&sc).unwrap();
        assert!(json.contains("\"directseller.com\""));
        assert!(json.contains("\"00001\""));
        let decoded: SupplyChain = serde_json::from_str(&json).unwrap();
        assert_eq!(sc, decoded);
    }

    #[test]
    fn supply_chain_multi_node_roundtrip() {
        let sc = SupplyChain {
            complete: 1,
            ver: "1.0".to_string(),
            nodes: vec![
                SupplyChainNode {
                    asi: "publisher.com".to_string(),
                    sid: "pub-001".to_string(),
                    hp: Some(1),
                    ..Default::default()
                },
                SupplyChainNode {
                    asi: "intermediary-ssp.com".to_string(),
                    sid: "ssp-account-7".to_string(),
                    rid: Some("req-abc".to_string()),
                    hp: Some(1),
                    ..Default::default()
                },
                SupplyChainNode {
                    asi: "exchange.com".to_string(),
                    sid: "exchange-pub-99".to_string(),
                    hp: Some(1),
                    ..Default::default()
                },
            ],
            ext: None,
        };
        let json = serde_json::to_string(&sc).unwrap();
        let decoded: SupplyChain = serde_json::from_str(&json).unwrap();
        assert_eq!(sc, decoded);
        assert_eq!(decoded.nodes.len(), 3);
        assert_eq!(decoded.nodes[1].rid, Some("req-abc".to_string()));
    }

    #[test]
    fn supply_chain_incomplete_flag() {
        let sc = SupplyChain {
            complete: 0,
            ver: "1.0".to_string(),
            nodes: vec![SupplyChainNode {
                asi: "known-ssp.com".to_string(),
                sid: "known-account".to_string(),
                hp: Some(1),
                ..Default::default()
            }],
            ext: None,
        };
        let json = serde_json::to_string(&sc).unwrap();
        assert!(json.contains("\"complete\":0"));
        let decoded: SupplyChain = serde_json::from_str(&json).unwrap();
        assert_eq!(sc.complete, decoded.complete);
    }

    #[test]
    fn supply_chain_ext_roundtrip() {
        let sc = SupplyChain {
            complete: 1,
            ver: "1.0".to_string(),
            nodes: vec![],
            ext: Some(serde_json::json!({ "debug": "trace" })),
        };
        let json = serde_json::to_string(&sc).unwrap();
        assert!(json.contains("\"ext\":{\"debug\":\"trace\"}"));
        let decoded: SupplyChain = serde_json::from_str(&json).unwrap();
        assert_eq!(sc, decoded);
    }

    /// Verify that parsing a JSON supply chain object from a real-world-style
    /// string works end-to-end.
    #[test]
    fn supply_chain_parse_from_json_string() {
        let raw = r#"{
            "complete": 1,
            "ver": "1.0",
            "nodes": [
                {
                    "asi": "directseller.com",
                    "sid": "00001",
                    "hp": 1,
                    "name": "Direct Seller LLC",
                    "domain": "directseller.com"
                },
                {
                    "asi": "exchange.com",
                    "sid": "exchange-1",
                    "rid": "bid-request-id-1",
                    "hp": 1
                }
            ]
        }"#;

        let sc: SupplyChain = serde_json::from_str(raw).unwrap();
        assert_eq!(sc.complete, 1);
        assert_eq!(sc.ver, "1.0");
        assert_eq!(sc.nodes.len(), 2);

        let first = &sc.nodes[0];
        assert_eq!(first.asi, "directseller.com");
        assert_eq!(first.sid, "00001");
        assert_eq!(first.hp, Some(1));
        assert_eq!(first.name.as_deref(), Some("Direct Seller LLC"));
        assert_eq!(first.domain.as_deref(), Some("directseller.com"));

        let second = &sc.nodes[1];
        assert_eq!(second.asi, "exchange.com");
        assert_eq!(second.sid, "exchange-1");
        assert_eq!(second.rid.as_deref(), Some("bid-request-id-1"));
        assert_eq!(second.hp, Some(1));
    }
}
