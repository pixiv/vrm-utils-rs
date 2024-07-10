use vrm_spec::{vrm_0_0, vrmc_spring_bone_1_0, vrmc_vrm_1_0};

#[test]
fn test_vrm0() {
    let file = include_bytes!("../../../fixtures/AvatarSample_A.vrm");
    let (doc, _, _) = gltf::import_slice(file).expect("ok");
    let extensions = doc.extension_value(vrm_0_0::VRM).expect("exist");
    let vrm: vrm_0_0::VRM0Schema = serde_json::from_value(extensions.to_owned()).expect("ok");

    insta::assert_debug_snapshot!(vrm);
}

#[test]
fn test_vrm1() {
    let file = include_bytes!("../../../fixtures/VRM1_Constraint_Twist_Sample.vrm");
    let (doc, _, _) = gltf::import_slice(file).expect("ok");
    let value = doc.extension_value(vrmc_vrm_1_0::VRMC_VRM).expect("exist");
    let vrmc_vrm: vrmc_vrm_1_0::VRMCVrmSchema =
        serde_json::from_value(value.to_owned()).expect("ok");

    insta::assert_debug_snapshot!(vrmc_vrm);

    let value = doc
        .extension_value(vrmc_spring_bone_1_0::VRMC_SPRING_BONE)
        .expect("exist");

    let vrmc_spring_bone: vrmc_spring_bone_1_0::VrmcSpringBoneSchema =
        serde_json::from_value(value.to_owned()).expect("ok");

    insta::assert_debug_snapshot!(vrmc_spring_bone);
}
