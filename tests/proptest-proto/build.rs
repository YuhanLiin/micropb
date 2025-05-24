use std::path::PathBuf;

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

    let config = pb_rs::types::Config {
        in_file: PathBuf::from("all_types.proto"),
        out_file: PathBuf::from(std::env::var("OUT_DIR").unwrap() + "/all_types_pbrs.rs"),
        single_module: true,
        import_search_path: vec![PathBuf::from(".")],
        no_output: false,
        error_cycle: false,
        headers: false,
        dont_use_cow: false,
        custom_struct_derive: vec![],
        custom_repr: None,
        custom_rpc_generator: Box::new(|_, _| Ok(())),
        custom_includes: Vec::new(),
        owned: false,
        hashbrown: false,
        nostd: false,
        gen_info: false,
        add_deprecated_fields: false,
    };
    pb_rs::types::FileDescriptor::write_proto(&config).unwrap();
}
