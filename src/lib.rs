//! # quake_qtvtinfo
//! Parse QuakeWorld qtvtinfo strings

use std::collections::HashMap;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Qtvinfo {
    pub hostname: Option<String>,
    pub maxclients: Option<u32>,
    pub version: Option<String>,
}

/// # Examples
/// ```
/// use quake_qtvinfo::Qtvinfo;
///
/// let info = Qtvinfo::from(r#"\hostname\QUAKE.SE KTX Qtv\maxclients\100\*version\QTV 1.14"#);
/// assert_eq!(info.version, Some("QTV 1.14".to_string()));
/// assert_eq!(info.maxclients, Some(100));
/// assert_eq!(info.hostname, Some("QUAKE.SE KTX Qtv".to_string()));
/// ```
impl From<&str> for Qtvinfo {
    fn from(value: &str) -> Self {
        Self::from(&quake_infostring::to_hashmap(value))
    }
}

/// # Examples
/// ```
/// use std::collections::HashMap;
/// use quake_qtvinfo::Qtvinfo;
///
/// let map = HashMap::from([
///     ("hostname".to_string(), "QUAKE.SE KTX Qtv".to_string()),
///     ("maxclients".to_string(), "100".to_string()),
///     ("*version".to_string(), "QTV 1.14".to_string()),
/// ]);
/// let info = Qtvinfo::from(&map);
/// assert_eq!(info.version, Some("QTV 1.14".to_string()));
/// assert_eq!(info.maxclients, Some(100));
/// assert_eq!(info.hostname, Some("QUAKE.SE KTX Qtv".to_string()));
/// ```
impl From<&HashMap<String, String>> for Qtvinfo {
    fn from(value: &HashMap<String, String>) -> Self {
        Self {
            hostname: map_get_string(value, "hostname"),
            maxclients: map_get_u32(value, "maxclients"),
            version: map_get_string(value, "*version"),
        }
    }
}

fn map_get_string(map: &HashMap<String, String>, key: &str) -> Option<String> {
    map.get(key).map(|v| v.to_string())
}

fn map_get_u32(map: &HashMap<String, String>, key: &str) -> Option<u32> {
    map.get(key)?.parse().ok()
}
