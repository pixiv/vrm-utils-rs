//! Data structures for the [`VRMC_vrm_animation`](https://github.com/vrm-c/vrm-specification/blob/master/specification/VRMC_vrm_animation-1.0/README.md) 1.0 glTF Extension.

#[cfg(feature = "rustc_hash")]
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "rustc_hash"))]
use std::collections::HashMap;

/// VRMC_vrm_animation extension name
pub const VRMC_VRM_ANIMATION: &str = "VRMC_vrm_animation";

/// glTF extension that defines humanoid animations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcVrmAnimationSchema {
    /// Specification version of VRMC_vrm_animation
    pub spec_version: String,

    /// An object which describes about humanoid bones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humanoid: Option<VrmcVrmAnimationHumanoid>,

    /// An object which maps expressions to nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expressions: Option<VrmcVrmAnimationExpressions>,

    /// An object which maps a eye gaze point to a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at: Option<VrmcVrmAnimationLookAt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// An object which describes about humanoid bones.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcVrmAnimationHumanoid {
    /// An object which maps humanoid bones to nodes.
    pub human_bones: VrmcVrmAnimationHumanBones,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Represents a set of humanBones of a humanoid.
// FIXME: all bones are optional by now
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrmcVrmAnimationHumanBones(
    pub HashMap<crate::vrmc_vrm_1_0::HumanBoneName, Option<VrmcVrmAnimationHumanBone>>,
);

/// Represents a single bone of a Humanoid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrmcVrmAnimationHumanBone {
    /// Represents a single glTF node tied to this humanBone.
    #[cfg(feature = "gltf_index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// An object which maps expressions to nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrmcVrmAnimationExpressions {
    /// An object that contains definitions of preset expressions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<VrmcVrmAnimationExpressionPreset>,

    /// An object that contains definitions of custom expressions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, VrmcVrmAnimationExpression>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// An object that contains definitions of preset expressions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrmcVrmAnimationExpressionPreset(
    pub HashMap<crate::vrmc_vrm_1_0::ExpressionPresetName, VrmcVrmAnimationExpression>,
);

/// Represents a single expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrmcVrmAnimationExpression {
    /// Represents a single glTF node mapped to this expression.
    #[cfg(feature = "gltf_index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// An object which maps a eye gaze point to a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcVrmAnimationLookAt {
    /// Represents a single glTF node represents the eye gaze point.
    #[cfg(feature = "gltf_index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<usize>,

    /// The position offset of the origin of the LookAt from the humanoid head bone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_from_head_bone: Option<[f64; 3]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}
