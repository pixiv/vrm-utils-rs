//! Data structures for the [`VRMC_springBone`](https://github.com/vrm-c/vrm-specification/tree/master/specification/VRMC_springBone-1.0) 1.0 glTF Extension.

#[cfg(feature = "rustc_hash")]
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "rustc_hash"))]
use std::collections::HashMap;

/// VRMC_springBone extension name
pub const VRMC_SPRING_BONE: &str = "VRMC_springBone";

/// SpringBone makes objects such as costumes and hair swaying
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcSpringBoneSchema {
    /// An array of colliderGroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collider_groups: Option<Vec<ColliderGroup>>,

    /// An array of colliders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colliders: Option<Vec<Collider>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// Specification version of VRMC_springBone
    pub spec_version: String,

    /// An array of springs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub springs: Option<Vec<Spring>>,
}

/// collider group definition for SpringBone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColliderGroup {
    /// An array of colliders.
    pub colliders: Vec<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// Name of the ColliderGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// collider definition for SpringBone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// The node index.
    #[cfg(feature = "gltf_index")]
    pub node: gltf::json::Index<gltf::json::Node>,
    #[cfg(not(feature = "gltf_index"))]
    pub node: usize,

    pub shape: ColliderShape,
}

/// Shape of collider. Have one of sphere and capsule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColliderShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capsule: Option<ColliderShapeCapsule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sphere: Option<ColliderShapeSphere>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColliderShapeCapsule {
    /// The capsule head. vector3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 3]>,

    /// The capsule radius
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,

    /// The capsule tail. vector3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<[f64; 3]>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColliderShapeSphere {
    /// The sphere center. vector3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 3]>,

    /// The sphere radius
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

/// A bone group of VRMCSpringBone.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spring {
    /// An index of node which is used as a root of center space.
    #[cfg(feature = "gltf_index")]
    pub center: Option<gltf::json::Index<gltf::json::Node>>,
    #[cfg(not(feature = "gltf_index"))]
    pub center: Option<usize>,

    /// Indices of ColliderGroups that detect collision with this spring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collider_groups: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// Joints of the spring. Except for the first element, a previous joint of the array must be
    /// an ancestor of the joint.
    pub joints: Vec<SpringBoneJoint>,

    /// Name of the Spring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A bone joint of VRMCSpringBone.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpringBoneJoint {
    /// Air resistance. Deceleration force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_force: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,

    /// The direction of gravity. A gravity other than downward direction also works.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity_dir: Option<[f64; 3]>,

    /// Gravitational acceleration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity_power: Option<f64>,

    /// The radius of spring sphere.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_radius: Option<f64>,

    /// The node index.
    #[cfg(feature = "gltf_index")]
    pub node: gltf::json::Index<gltf::json::Node>,
    #[cfg(not(feature = "gltf_index"))]
    pub node: usize,

    /// The force to return to the initial pose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stiffness: Option<f64>,
}
