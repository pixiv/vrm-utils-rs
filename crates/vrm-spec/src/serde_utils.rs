/// This mod handles common issues when using VRMs.

#[cfg(feature = "rustc_hash")]
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Deserializer};
#[cfg(not(feature = "rustc_hash"))]
use std::collections::HashMap;

#[cfg(feature = "gltf_index")]
use gltf::json::Index;

// NOTE: Unity puts -1 in some indexes but those are not valid as glTF index.
// Treat minus values as None
#[cfg(feature = "gltf_index")]
pub(crate) fn deserialize_option_index<'de, D, T>(
    deserializer: D,
) -> Result<Option<Index<T>>, D::Error>
where
    D: Deserializer<'de>,
{
    let n = i64::deserialize(deserializer)?;
    Ok(if n >= 0 {
        Some(Index::new(n as u32))
    } else {
        None
    })
}

// NOTE: Unity puts -1 in some indexes but those are not valid as glTF index.
// Treat minus values as None
#[cfg(feature = "gltf_index")]
pub(crate) fn deserialize_option_map_index<'de, D, T>(
    deserializer: D,
) -> Result<Option<HashMap<String, gltf::json::Index<T>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map = HashMap::<String, i64>::deserialize(deserializer)?;
    Ok(Some(HashMap::<String, gltf::json::Index<T>>::from_iter(
        map.into_iter().filter_map(|(k, v)| {
            if v >= 0 {
                Some((k, Index::<T>::new(v as u32)))
            } else {
                None
            }
        }),
    )))
}
