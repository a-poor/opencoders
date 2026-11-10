use super::{ModelInfo, ModelVariant};
use serde::Deserialize;

impl std::fmt::Debug for super::ProviderSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ProviderSettings { .. }")
    }
}

impl std::fmt::Debug for ModelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelInfo { .. }")
    }
}
impl std::fmt::Debug for ModelVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelVariant { .. }")
    }
}

pub(super) fn deserialize_model_integer<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<i64, D::Error> {
    let value = i64::deserialize(d)?;
    if !(-9_007_199_254_740_991..=9_007_199_254_740_991).contains(&value) {
        return Err(serde::de::Error::custom(
            "model integer exceeds the signed safe-integer range",
        ));
    }
    Ok(value)
}
pub(super) fn deserialize_optional_model_integer<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<i64>, D::Error> {
    deserialize_model_integer(d).map(Some)
}
// A custom deserializer without `default` makes this nullable field required.
pub(super) fn deserialize_nullable_model<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<ModelInfo>, D::Error> {
    Option::<ModelInfo>::deserialize(d)
}
