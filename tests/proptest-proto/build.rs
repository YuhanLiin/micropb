use micropb_gen::{Config, Generator};

fn main() {
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
        .compile_protos(
            &["all_types.proto"],
            std::env::var("OUT_DIR").unwrap() + "/all_types.rs",
        )
        .unwrap();
}
