use micropb_gen::{Config, Generator};

fn micropb_configured() -> Generator {
    let mut generator = Generator::new();
    // Can only derive Arbitrary on standard containers, so use containers from alloc
    generator
        .use_container_alloc()
        .configure(
            ".",
            Config::new().type_attributes("#[derive(proptest_derive::Arbitrary)]"),
        )
        // Boxed optional fields will use `Option` instead of hazzers
        .configure(".TestTypesOptional2", Config::new().boxed(true));
    generator
}

fn main() {
    // Normal micropb module
    micropb_configured()
        .compile_protos(
            &["all_types.proto"],
            std::env::var("OUT_DIR").unwrap() + "/micropb_all_types.rs",
        )
        .unwrap();

    // Generate the test oneof as an enum
    let mut gen = micropb_configured();
    gen.single_oneof_msg_as_enum(true);
    gen.compile_protos(
        &["all_types.proto"],
        std::env::var("OUT_DIR").unwrap() + "/micropb_oneof_enum.rs",
    )
    .unwrap();

    // Prost module
    prost_build::compile_protos(&["all_types.proto"], &["."]).unwrap();
}
