# openrtb26

Rust types for the [OpenRTB 2.6](https://iabtechlab.com/standards/openrtb/) specification with full [serde](https://serde.rs/) JSON support.

## Features

- Complete coverage of the OpenRTB 2.6 bid request and response objects
- Serialize / deserialize with `serde_json` out of the box
- Spec-correct default values (e.g. `at = 2`, `cur = "USD"`, `cattax = 1`)
- Optional fields are skipped when serializing to keep payloads lean
- Unknown fields are silently ignored during deserialization for forward-compatibility
- All types derive `Debug`, `Clone`, and `PartialEq`

## Quick start

Add to your `Cargo.toml`:

```toml
[dependencies]
openrtb26 = "0.1"
serde_json = "1"
```

### Build a bid request

```rust
use openrtb26::{BidRequest, Imp, Banner, Site};

let request = BidRequest {
    id: "auction-1234".to_string(),
    imp: vec![Imp {
        id: "1".to_string(),
        banner: Some(Banner {
            w: Some(300),
            h: Some(250),
            ..Default::default()
        }),
        ..Default::default()
    }],
    site: Some(Site {
        domain: Some("example.com".to_string()),
        page: Some("https://example.com/article".to_string()),
        ..Default::default()
    }),
    ..Default::default()
};

let json = serde_json::to_string(&request).unwrap();
```

### Parse a bid response

```rust
use openrtb26::BidResponse;

let raw = r#"{
    "id": "auction-1234",
    "seatbid": [{
        "bid": [{
            "id": "1",
            "impid": "1",
            "price": 4.50,
            "adm": "<div>ad markup</div>",
            "adomain": ["advertiser.com"]
        }]
    }]
}"#;

let response: BidResponse = serde_json::from_str(raw).unwrap();
assert_eq!(response.seatbid.unwrap()[0].bid[0].price, 4.50);
```

## Type mapping

All types are re-exported from the crate root for ergonomic imports (`use openrtb26::Bid`).

| Type | Spec Section | Module |
|------|-------------|--------|
| `BidRequest` | 3.2.1 | `lib` |
| `Source` | 3.2.2 | `source` |
| `Regs` | 3.2.3 | `source` |
| `Imp` | 3.2.4 | `imp` |
| `Metric` | 3.2.5 | `imp` |
| `Video` | 3.2.7 | `video` |
| `Audio` | 3.2.8 | `audio` |
| `Native` | 3.2.9 | `native` |
| `Banner` | 3.2.10 | `banner` |
| `Format` | 3.2.10 | `format` |
| `Pmp` | 3.2.11 | `pmp` |
| `Deal` | 3.2.12 | `pmp` |
| `Site` | 3.2.13 | `site` |
| `App` | 3.2.14 | `site` |
| `Publisher` | 3.2.15 | `segment` |
| `Content` | 3.2.16 | `content` |
| `Producer` | 3.2.17 | `segment` |
| `Device` | 3.2.18 | `device` |
| `Geo` | 3.2.19 | `geo` |
| `User` | 3.2.20 | `segment` |
| `Data` | 3.2.21 | `segment` |
| `Segment` | 3.2.22 | `segment` |
| `Network` | 3.2.23 | `segment` |
| `Channel` | 3.2.24 | `segment` |
| `SupplyChain` | 3.2.25 | `supply_chain` |
| `SupplyChainNode` | 3.2.25 | `supply_chain` |
| `DurFloors` | 3.2.26 | `source` |
| `Eid` | 3.2.27 | `eid` |
| `Uid` | 3.2.28 | `eid` |
| `UserAgent` | 3.2.29 | `user_agent` |
| `BrandVersion` | 3.2.30 | `user_agent` |
| `Qty` | 3.2.31 | `source` |
| `Dooh` | 3.2.32 | `site` |
| `Refresh` | 3.2.33 | `segment` |
| `RefSettings` | 3.2.34 | `segment` |
| `BidResponse` | 4.2.1 | `lib` |
| `SeatBid` | 4.2.2 | `seat_bid` |
| `Bid` | 4.2.3 | `bid` |

## License

MIT
