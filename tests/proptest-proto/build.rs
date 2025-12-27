use micropb_gen::{Config, Generator};

fn micropb_configured(oneof_as_enum: bool, encode_cache: bool) -> Generator {
    let mut generator = Generator::new();
    // Can only derive Arbitrary on standard containers, so use containers from alloc
    generator
        .single_oneof_msg_as_enum(oneof_as_enum)
        .encode_cache(encode_cache)
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
    micropb_configured(false, false)
        .compile_protos(
            &["all_types.proto"],
            std::env::var("OUT_DIR").unwrap() + "/micropb_all_types.rs",
        )
        .unwrap();

    // Generate the test oneof as an enum
    micropb_configured(true, false)
        .compile_protos(
            &["all_types.proto"],
            std::env::var("OUT_DIR").unwrap() + "/micropb_oneof_enum.rs",
        )
        .unwrap();

    // With encode caching
    micropb_configured(false, true)
        .compile_protos(
            &["all_types.proto"],
            std::env::var("OUT_DIR").unwrap() + "/micropb_all_types.cached.rs",
        )
        .unwrap();

    // Generate the test oneof as an enum with encode caching
    micropb_configured(true, true)
        .compile_protos(
            &["all_types.proto"],
            std::env::var("OUT_DIR").unwrap() + "/micropb_oneof_enum.cached.rs",
        )
        .unwrap();

    // Prost module
    prost_build::compile_protos(&["all_types.proto"], &["."]).unwrap();
}
