//! Data structures for the [`VRMC_vrm`](https://github.com/vrm-c/vrm-specification/tree/master/specification/VRMC_vrm-1.0) 1.0 glTF Extension.

pub mod expression_preset_name;
pub mod human_bone_name;

pub use expression_preset_name::ExpressionPresetName;
pub use human_bone_name::HumanBoneName;
use serde::{Deserialize, Serialize};

/// VRMC_VRM extension name
pub const VRMC_VRM: &str = "VRMC_vrm";

#[cfg(feature = "rustc_hash")]
use rustc_hash::FxHashMap as HashMap;
#[cfg(not(feature = "rustc_hash"))]
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMCVrmSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expressions: Option<Expressions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// First-person perspective settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_person: Option<FirstPerson>,

    pub humanoid: Humanoid,

    /// Eye gaze control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at: Option<LookAt>,

    /// Meta informations of the VRM model
    pub meta: Meta,

    /// Specification version of VRMC_vrm
    pub spec_version: String,
}

/// Definition of expressions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expressions {
    /// Custom expressions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, Expression>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// Preset expressions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

/// Definition of expression by weighted animation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// A value greater than 0.5 is 1.0, otherwise 0.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_binary: Option<bool>,

    /// Material color animation references
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_color_binds: Option<Vec<MaterialColorBind>>,

    /// Specify a morph target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_target_binds: Option<Vec<MorphTargetBind>>,

    /// Override values of Blink expressions when this Expression is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_blink: Option<ExpressionOverrideType>,

    /// Override values of LookAt expressions when this Expression is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_look_at: Option<ExpressionOverrideType>,

    /// Override values of Mouth expressions when this Expression is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_mouth: Option<ExpressionOverrideType>,

    /// Texture transform animation references
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_transform_binds: Option<Vec<TextureTransformBind>>,
}

/// Material color value associated with a expression
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialColorBind {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// target material
    #[cfg(feature = "gltf_index")]
    pub material: gltf::json::Index<gltf::json::Material>,
    #[cfg(not(feature = "gltf_index"))]
    pub material: usize,

    /// target color
    pub target_value: Vec<f64>,

    pub material_color_bind_type: MaterialColorType,
}

/// Morph target value associated with a expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphTargetBind {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// The index of the morph target in the mesh.
    pub index: usize,

    /// The index of the node that attached to target mesh.
    #[cfg(feature = "gltf_index")]
    pub node: gltf::json::Index<gltf::json::Node>,
    #[cfg(not(feature = "gltf_index"))]
    pub node: usize,

    /// The weight value of target morph target.
    pub weight: f64,
}

/// Texture transform value associated with a expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureTransformBind {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// target material
    #[cfg(feature = "gltf_index")]
    pub material: gltf::json::Index<gltf::json::Node>,
    #[cfg(not(feature = "gltf_index"))]
    pub material: usize,

    /// uv offset for TEXCOORD_0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Vec<f64>>,

    /// uv scaling for TEXCOORD_0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Vec<f64>>,
}
/// Preset expressions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset(pub HashMap<ExpressionPresetName, Expression>);

/// First-person perspective settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstPerson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// Mesh rendering annotation for cameras.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_annotations: Option<Vec<MeshAnnotation>>,
}

/// Specify how the mesh should be interpreted by the camera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshAnnotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// The index of the node that attached to target mesh.
    #[cfg(feature = "gltf_index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<usize>,

    /// How the camera interprets the mesh.
    #[serde(rename = "type")]
    pub mesh_annotation_type: FirstPersonType,
}

/// Correspondence between nodes and human bones
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Humanoid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    pub human_bones: HumanBones,
}

/// Represents a set of humanBones of a humanoid.
// FIXME: all bones are optional by now
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanBones(pub HashMap<HumanBoneName, Option<HumanBone>>);

/// Represents a single bone of a Humanoid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanBone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// Represents a single glTF node tied to this humanBone.
    #[cfg(feature = "gltf_index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<usize>,
}

/// Eye gaze control
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookAt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// The origin of LookAt. Position offset from the head bone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_from_head_bone: Option<Vec<f64>>,

    /// Horizontal inward movement. The left eye moves right. The right eye moves left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_map_horizontal_inner: Option<LookAtRangeMap>,

    /// Horizontal outward movement. The left eye moves left. The right eye moves right.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_map_horizontal_outer: Option<LookAtRangeMap>,

    /// Vertical downward movement. Both eyes move upwards
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_map_vertical_down: Option<LookAtRangeMap>,

    /// Vertical upward movement. Both eyes move downwards
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_map_vertical_up: Option<LookAtRangeMap>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at_type: Option<LookAtType>,
}

/// LookAt range definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookAtRangeMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// Yaw and pitch angles  ( degrees )  between the head bone forward vector and the eye gaze
    /// LookAt vector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_max_value: Option<f64>,

    /// Degree for type.bone, Weight for type.expressions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_scale: Option<f64>,
}

/// Meta information of the VRM model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// A flag that permits to use this model in contents contain anti-social activities or hate
    /// speeches
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_antisocial_or_hate_usage: Option<bool>,

    /// A flag that permits to use this model in excessively sexual contents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_excessively_sexual_usage: Option<bool>,

    /// A flag that permits to use this model in excessively violent contents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_excessively_violent_usage: Option<bool>,

    /// A flag that permits to use this model in political or religious contents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_political_or_religious_usage: Option<bool>,

    /// A flag that permits to redistribute this model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redistribution: Option<bool>,

    /// Authors of the model
    pub authors: Vec<String>,

    /// A person who can perform as an avatar with this model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_permission: Option<AvatarPermissionType>,

    /// An option that permits to use this model in commercial products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commercial_usage: Option<CommercialUsageType>,

    /// An information that describes the contact information of the author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_information: Option<String>,

    /// An information that describes the copyright of the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_information: Option<String>,

    /// An option that forces or abandons to display the credit of this model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_notation: Option<CreditNotationType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// A URL towards the license document this model refers to
    pub license_url: String,

    /// An option that controls the condition to modify this model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification: Option<ModificationType>,

    /// The name of the model
    pub name: String,

    /// Describe the URL links of other license
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_license_url: Option<String>,

    /// References / original works of the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<String>>,

    /// Third party licenses of the model, if required. You can use line breaks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_licenses: Option<String>,

    /// The index to the thumbnail image of the model in gltf.images
    #[cfg(feature = "gltf_index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_image: Option<gltf::json::Index<gltf::json::Image>>,
    #[cfg(not(feature = "gltf_index"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_image: Option<usize>,

    /// The version of the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MaterialColorType {
    Color,
    EmissionColor,
    MatcapColor,
    OutlineColor,
    RimColor,
    ShadeColor,
}

/// Expression override types
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExpressionOverrideType {
    Blend,
    Block,
    None,
}

/// How the camera interprets the mesh.
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FirstPersonType {
    Auto,
    Both,
    FirstPersonOnly,
    ThirdPersonOnly,
}

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LookAtType {
    Bone,
    Expression,
}

/// A person who can perform as an avatar with this model
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AvatarPermissionType {
    Everyone,
    OnlyAuthor,
    OnlySeparatelyLicensedPerson,
}

/// An option that permits to use this model in commercial products
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommercialUsageType {
    Corporation,
    PersonalNonProfit,
    PersonalProfit,
}

/// An option that forces or abandons to display the credit of this model
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CreditNotationType {
    Required,
    Unnecessary,
}

/// An option that controls the condition to modify this model
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModificationType {
    AllowModification,
    AllowModificationRedistribution,
    Prohibited,
}
