//! Data structures for the [`VRM`](https://github.com/vrm-c/vrm-specification/tree/master/specification/0.0) 0.0 glTF Extension.

/// VRM extension name
pub const VRM: &str = "VRM";

#[cfg(feature = "rustc_hash")]
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "rustc_hash"))]
use std::collections::HashMap;

use crate::serde_utils::{
    deserialize_option_index, deserialize_option_map_and_skip_nullable,
    deserialize_option_map_index,
};

/// VRM extension is for 3d humanoid avatars (and models) in VR applications.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRM0Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_shape_master: Option<VRMBlendShape>,

    /// Version of exporter that vrm created. UniVRM-0.46
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exporter_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_person: Option<VRMFirstPerson>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub humanoid: Option<VRMHumanoid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_properties: Option<Vec<VRMMaterial>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<VRMMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_animation: Option<VRMSecondaryAnimation>,

    /// Version of VRM specification. 0.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_version: Option<String>,
}

/// BlendShapeAvatar of UniVRM
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMBlendShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_shape_groups: Option<Vec<VRMBlendShapeGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMBlendShapeGroup {
    /// Low level blendshape references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binds: Option<Vec<VRMBlendShapeBind>>,

    /// 0 or 1. Do not allow an intermediate value. Value should rounded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_binary: Option<bool>,

    /// Material animation references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_values: Option<Vec<VRMBlendShapeMaterialBind>>,

    /// Expression name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Predefined Expression name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_name: Option<PresetName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VRMBlendShapeBind {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_index::<_, gltf::json::Mesh>"
    )]
    #[cfg(feature = "gltf_index")]
    pub mesh: Option<gltf::json::Index<gltf::json::Mesh>>,
    #[cfg(not(feature = "gltf_index"))]
    pub mesh: Option<i64>,

    /// SkinnedMeshRenderer.SetBlendShapeWeight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMBlendShapeMaterialBind {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMFirstPerson {
    /// The bone whose rendering should be turned off in first-person view. Usually Head is
    /// specified.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_index::<_, gltf::json::Node>"
    )]
    #[cfg(feature = "gltf_index")]
    pub first_person_bone: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    pub first_person_bone: Option<i64>,

    /// The target position of the VR headset in first-person view. It is assumed that an offset
    /// from the head bone to the VR headset is added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_person_bone_offset: Option<FirstPersonBoneOffset>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at_horizontal_inner: Option<VRMFirstPersonDegreeMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at_horizontal_outer: Option<VRMFirstPersonDegreeMap>,

    /// Eye controller mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at_type_name: Option<LookAtTypeName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at_vertical_down: Option<VRMFirstPersonDegreeMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at_vertical_up: Option<VRMFirstPersonDegreeMap>,

    /// Switch display / undisplay for each mesh in first-person view or the others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_annotations: Option<Vec<VRMFirstPersonMeshAnnotation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Vector3 but has optional x,y,z.
///
/// normally x,y,z should not be optional but the VRM 0.0 spec allows it
pub struct OptionalVector3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

/// The target position of the VR headset in first-person view. It is assumed that an offset
/// from the head bone to the VR headset is added.
pub type FirstPersonBoneOffset = OptionalVector3;

/// Eye controller setting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMFirstPersonDegreeMap {
    /// None linear mapping params. time, value, inTangent, outTangent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve: Option<Vec<f64>>,

    /// Look at input clamp range degree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_range: Option<f64>,

    /// Look at map range degree from xRange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_range: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMFirstPersonMeshAnnotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_person_flag: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_index::<_, gltf::json::Mesh>"
    )]
    #[cfg(feature = "gltf_index")]
    pub mesh: Option<gltf::json::Index<gltf::json::Mesh>>,
    #[cfg(not(feature = "gltf_index"))]
    pub mesh: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMHumanoid {
    /// Unity's HumanDescription.armStretch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_stretch: Option<f64>,

    /// Unity's HumanDescription.feetSpacing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feet_spacing: Option<f64>,

    /// Unity's HumanDescription.hasTranslationDoF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_translation_do_f: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_bones: Option<Vec<VRMHumanoidBone>>,

    /// Unity's HumanDescription.legStretch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leg_stretch: Option<f64>,

    /// Unity's HumanDescription.lowerArmTwist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_arm_twist: Option<f64>,

    /// Unity's HumanDescription.lowerLegTwist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_leg_twist: Option<f64>,

    /// Unity's HumanDescription.upperArmTwist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_arm_twist: Option<f64>,

    /// Unity's HumanDescription.upperLegTwist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_leg_twist: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMHumanoidBone {
    /// Unity's HumanLimit.axisLength
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_length: Option<f64>,

    /// Human bone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bone: Option<Bone>,

    /// Unity's HumanLimit.center
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Center>,

    /// Unity's HumanLimit.max
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Max>,

    /// Unity's HumanLimit.min
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Min>,

    /// Reference node index
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_index::<_, gltf::json::Node>"
    )]
    #[cfg(feature = "gltf_index")]
    pub node: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    pub node: Option<i64>,

    /// Unity's HumanLimit.useDefaultValues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_values: Option<bool>,
}

/// Unity's HumanLimit.center
pub type Center = OptionalVector3;

/// Unity's HumanLimit.max
pub type Max = OptionalVector3;

/// Unity's HumanLimit.min
pub type Min = OptionalVector3;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMMaterial {
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_map_and_skip_nullable::<_, String, f64>"
    )]
    pub float_properties: Option<HashMap<String, f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_map: Option<HashMap<String, bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_queue: Option<i64>,

    /// This contains shader name.  VRM/MToon, VRM/UnlitTransparentZWrite, and VRM_USE_GLTFSHADER
    /// (and legacy materials as Standard, UniGLTF/UniUnlit, VRM/UnlitTexture, VRM/UnlitCutout,
    /// VRM/UnlitTransparent) . If VRM_USE_GLTFSHADER is specified, use same index of gltf's
    /// material settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shader: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_map: Option<HashMap<String, String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_map_index::<_, gltf::json::Texture>"
    )]
    #[cfg(feature = "gltf_index")]
    pub texture_properties: Option<HashMap<String, gltf::json::Index<gltf::json::Texture>>>,
    #[cfg(not(feature = "gltf_index"))]
    pub texture_properties: Option<HashMap<String, usize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_properties: Option<HashMap<String, Vec<f64>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMMeta {
    /// A person who can perform with this avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_user_name: Option<AllowedUserName>,

    /// Author of VRM model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// For commercial use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commercial_ussage_name: Option<UssageName>,

    /// Contact Information of VRM model author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_information: Option<String>,

    /// License type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_name: Option<LicenseName>,

    /// If “Other” is selected, put the URL link of the license document here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_license_url: Option<String>,

    /// If there are any conditions not mentioned above, put the URL link of the license document
    /// here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_permission_url: Option<String>,

    /// Reference of VRM model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Permission to perform sexual acts with this avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexual_ussage_name: Option<UssageName>,

    /// Thumbnail of VRM model
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_index::<_, gltf::json::Texture>"
    )]
    #[cfg(feature = "gltf_index")]
    pub texture: Option<gltf::json::Index<gltf::json::Texture>>,
    #[cfg(not(feature = "gltf_index"))]
    pub texture: Option<i64>,

    /// Title of VRM model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Version of VRM model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Permission to perform violent acts with this avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violent_ussage_name: Option<UssageName>,
}

/// The setting of automatic animation of string-like objects such as tails and hairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMSecondaryAnimation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bone_groups: Option<Vec<VRMSecondaryAnimationSpring>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collider_groups: Option<Vec<VRMSecondaryAnimationColliderGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRMSecondaryAnimationSpring {
    /// Specify the node index of the root bone of the swaying object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "gltf_index")]
    pub bones: Option<Vec<gltf::json::Index<gltf::json::Node>>>,
    #[cfg(not(feature = "gltf_index"))]
    pub bones: Option<Vec<i64>>,

    /// The reference point of a swaying object can be set at any location except the origin.
    /// When implementing UI moving with warp, the parent node to move with warp can be specified
    /// if you don't want to make the object swaying with warp movement.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_index::<_, gltf::json::Node>"
    )]
    #[cfg(feature = "gltf_index")]
    pub center: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    pub center: Option<i64>,

    /// Specify the index of the collider group for collisions with swaying objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collider_groups: Option<Vec<i64>>,

    /// Annotation comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// The resistance (deceleration) of automatic animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_force: Option<f64>,

    /// The direction of gravity. Set (0, -1, 0) for simulating the gravity. Set (1, 0, 0) for
    /// simulating the wind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity_dir: Option<GravityDir>,

    /// The strength of gravity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity_power: Option<f64>,

    /// The radius of the sphere used for the collision detection with colliders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_radius: Option<f64>,

    /// The resilience of the swaying object (the power of returning to the initial pose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stiffiness: Option<f64>,
}

/// The direction of gravity. Set (0, -1, 0) for simulating the gravity. Set (1, 0, 0) for
/// simulating the wind.
pub type GravityDir = OptionalVector3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VRMSecondaryAnimationColliderGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colliders: Option<Vec<Collider>>,

    /// The node of the collider group for setting up collision detections.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_index::<_, gltf::json::Node>"
    )]
    pub node: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    pub node: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collider {
    /// The local coordinate from the node of the collider group in *left-handed* Y-up coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Offset>,

    /// The radius of the collider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

/// The local coordinate from the node of the collider group in *left-handed* Y-up coordinate.
pub type Offset = OptionalVector3;

/// Predefined Expression name.
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresetName {
    A,
    Angry,
    Blink,
    BlinkL,
    BlinkR,
    E,
    Fun,
    I,
    Joy,
    Lookdown,
    Lookleft,
    Lookright,
    Lookup,
    Neutral,
    O,
    Sorrow,
    U,
    Unknown,
}

/// Eye controller mode.
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum LookAtTypeName {
    BlendShape,
    Bone,
}

/// Human bone name.
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Bone {
    Chest,
    Head,
    Hips,
    Jaw,
    LeftEye,
    LeftFoot,
    LeftHand,
    LeftIndexDistal,
    LeftIndexIntermediate,
    LeftIndexProximal,
    LeftLittleDistal,
    LeftLittleIntermediate,
    LeftLittleProximal,
    LeftLowerArm,
    LeftLowerLeg,
    LeftMiddleDistal,
    LeftMiddleIntermediate,
    LeftMiddleProximal,
    LeftRingDistal,
    LeftRingIntermediate,
    LeftRingProximal,
    LeftShoulder,
    LeftThumbDistal,
    LeftThumbIntermediate,
    LeftThumbProximal,
    LeftToes,
    LeftUpperArm,
    LeftUpperLeg,
    Neck,
    RightEye,
    RightFoot,
    RightHand,
    RightIndexDistal,
    RightIndexIntermediate,
    RightIndexProximal,
    RightLittleDistal,
    RightLittleIntermediate,
    RightLittleProximal,
    RightLowerArm,
    RightLowerLeg,
    RightMiddleDistal,
    RightMiddleIntermediate,
    RightMiddleProximal,
    RightRingDistal,
    RightRingIntermediate,
    RightRingProximal,
    RightShoulder,
    RightThumbDistal,
    RightThumbIntermediate,
    RightThumbProximal,
    RightToes,
    RightUpperArm,
    RightUpperLeg,
    Spine,
    UpperChest,
}

/// A person who can perform with this avatar.
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum AllowedUserName {
    Everyone,
    ExplicitlyLicensedPerson,
    OnlyAuthor,
}

/// Usage Permission.
///
/// NOTE: the typo is intended following the spec.
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum UssageName {
    Allow,
    Disallow,
}

/// License type.
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum LicenseName {
    #[serde(rename = "CC0")]
    Cc0,

    #[serde(rename = "CC_BY")]
    CcBy,

    #[serde(rename = "CC_BY_NC")]
    CcByNc,

    #[serde(rename = "CC_BY_NC_ND")]
    CcByNcNd,

    #[serde(rename = "CC_BY_NC_SA")]
    CcByNcSa,

    #[serde(rename = "CC_BY_ND")]
    CcByNd,

    #[serde(rename = "CC_BY_SA")]
    CcBySa,

    #[serde(rename = "Redistribution_Prohibited")]
    RedistributionProhibited,

    Other,
}
