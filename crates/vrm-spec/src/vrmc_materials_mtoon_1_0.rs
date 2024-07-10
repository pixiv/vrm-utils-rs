//! Data structures for the [`VRMC_materials_mtoon`](https://github.com/vrm-c/vrm-specification/tree/master/specification/VRMC_materials_mtoon-1.0) 1.0 glTF Extension.

use serde::{Deserialize, Serialize};

/// VRMC_materials_mtoon extension name
pub const VRMC_MATERIALS_MTOON: &str = "VRMC_materials_mtoon";

#[cfg(feature = "rustc_hash")]
use rustc_hash::FxHashMap as HashMap;
#[cfg(not(feature = "rustc_hash"))]
use std::collections::HashMap;

#[cfg(feature = "gltf_index")]
use gltf::json::texture::Info as TextureInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcMaterialsMtoonSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gi_equalization_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcap_factor: Option<[f64; 3]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcap_texture: Option<TextureInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color_factor: Option<[f64; 3]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_lighting_mix_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_width_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_width_mode: Option<OutlineWidthMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_width_multiply_texture: Option<TextureInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parametric_rim_color_factor: Option<[f64; 3]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parametric_rim_fresnel_power_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parametric_rim_lift_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_queue_offset_number: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rim_lighting_mix_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rim_multiply_texture: Option<TextureInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shade_color_factor: Option<[f64; 3]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shade_multiply_texture: Option<TextureInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shading_shift_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shading_shift_texture: Option<ShadingShiftTextureInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shading_toony_factor: Option<f64>,

    /// Specification version of VRMC_materials_mtoon
    pub spec_version: String,

    /// enable depth buffer when `alphaMode` is `BLEND`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent_with_z_write: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv_animation_mask_texture: Option<TextureInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv_animation_rotation_speed_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv_animation_scroll_x_speed_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv_animation_scroll_y_speed_factor: Option<f64>,
}

/// Reference to a texture.
#[cfg(not(feature = "gltf_index"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    pub index: usize,

    /// The set index of texture's TEXCOORD attribute used for texture coordinate mapping.
    #[cfg(feature = "gltf_index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<gltf::json::Index<gltf::json::Accessor>>,
    #[cfg(not(feature = "gltf_index"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<usize>,
}

/// Reference to a texture.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShadingShiftTextureInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// The index of the texture.
    #[cfg(feature = "gltf_index")]
    pub index: gltf::json::Index<gltf::json::Texture>,
    #[cfg(not(feature = "gltf_index"))]
    pub index: usize,

    /// The scalar multiplier applied to the texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,

    /// The set index of texture's TEXCOORD attribute used for texture coordinate mapping.
    #[cfg(feature = "gltf_index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<gltf::json::Index<gltf::json::Accessor>>,
    #[cfg(not(feature = "gltf_index"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<usize>,
}

/// Outline
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutlineWidthMode {
    None,
    ScreenCoordinates,
    WorldCoordinates,
}
