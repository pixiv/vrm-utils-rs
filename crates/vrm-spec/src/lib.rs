//! # vrm-spec
//!
//! Data structures for the [VRM](https://vrm.dev) Format.
//!
//! Schema definitions: <https://github.com/vrm-c/vrm-specification>
//!
//! ## Example
//!
//! ```rust
//! use vrm_spec::vrmc_vrm_1_0::{VRMCVrmSchema, VRMC_VRM};
//!
//! let file = include_bytes!("../../../fixtures/VRM1_Constraint_Twist_Sample.vrm");
//! let (doc, _, _) = gltf::import_slice(file).expect("ok");
//! let value = doc.extension_value(VRMC_VRM).expect("exist");
//! let vrmc_vrm: VRMCVrmSchema = serde_json::from_value(value.to_owned()).expect("ok");
//!
//! // do something with vrm
//! ```

mod serde_utils;
pub mod vrm_0_0;
pub mod vrmc_materials_mtoon_1_0;
pub mod vrmc_spring_bone_1_0;
pub mod vrmc_vrm_1_0;
