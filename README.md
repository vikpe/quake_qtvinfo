# quake_qtvinfo [![Test](https://github.com/vikpe/quake_qtvinfo/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/quake_qtvinfo/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/quake_qtvinfo)](https://crates.io/crates/quake_qtvinfo) [![docs.rs](https://img.shields.io/docsrs/quake_qtvinfo)](https://docs.rs/quake_qtvinfo/)

> Parse QuakeWorld qtvinfo strings

## Usage

```rust
use quake_qtvinfo::Qtvinfo;

let info = Qtvinfo::from(r#"\hostname\QUAKE.SE KTX Qtv\maxclients\100\*version\QTV 1.14"#);
assert_eq!(info.version, Some("QTV 1.14".to_string()));
assert_eq!(info.maxclients, Some(100));
assert_eq!(info.hostname, Some("QUAKE.SE KTX Qtv".to_string()));
```

## Fields

```rust
pub struct Qtvinfo {
    pub hostname: Option<String>,
    pub maxclients: Option<u32>,
    pub version: Option<String>,
}
```

## See also

* [qtvstat](https://github.com/vikpe/qtvstat) - Get information from QTV servers
* [quake_clientinfo](https://github.com/vikpe/quake_clientinfo) - Parse QuakeWorld clientinfo strings
* [quake_serverinfo](https://github.com/vikpe/quake_serverinfo) - Parse QuakeWorld serverinfo strings
* [quake_infostring](https://github.com/vikpe/quake_infostring) - Parse QuakeWorld info strings
