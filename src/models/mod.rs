//! Typed API response models derived from the pinned OpenAPI schema.
//!
//! Server information preserves additional fields in flattened `extra` maps.
//! Optional nullable fields decode to `Option`; absent and null are equivalent
//! for these read-only response fields, not for future update requests.

#[rustfmt::skip]
#[allow(dead_code)] // Typify emits conversion helpers unused by these selected models.
mod generated;
pub use generated::{ServerInfo, ServerInfoCapabilities, ServerInfoPaths};

fn deserialize_pid<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    let pid = <u64 as serde::Deserialize>::deserialize(deserializer)?;
    if pid > 9_007_199_254_740_991 {
        return Err(serde::de::Error::custom(
            "pid exceeds the schema's safe-integer maximum",
        ));
    }
    Ok(pid)
}
