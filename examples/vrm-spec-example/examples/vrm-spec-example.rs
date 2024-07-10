use vrm_spec::vrmc_vrm_1_0::{Meta, VRMCVrmSchema, VRMC_VRM};

fn main() {
    let file = include_bytes!("../../../fixtures/VRM1_Constraint_Twist_Sample.vrm");
    let (doc, _, _) = gltf::import_slice(file).expect("ok");
    let extensions = doc.extension_value(VRMC_VRM).expect("exist");
    let vrm: VRMCVrmSchema = serde_json::from_value(extensions.to_owned()).expect("ok");

    let Meta {
        name,
        authors,
        allow_redistribution,
        copyright_information,
        commercial_usage,
        credit_notation,
        avatar_permission,
        ..
    } = vrm.meta;

    println!("Details of {}", name);
    println!("VRM spec version: {}", vrm.spec_version);
    println!("Authors");
    for author in authors {
        println!("- {}", author);
    }
    println!("Licence");
    println!(
        r#"- allow_redistribution: {}
- copyright_information: {}
- commercial_usage: {:?}
- credit_notation: {:?}
- avatar_permission: {:?}"#,
        allow_redistribution.unwrap_or(false),
        copyright_information.as_deref().unwrap_or("None"),
        commercial_usage,
        credit_notation,
        avatar_permission
    )
}
