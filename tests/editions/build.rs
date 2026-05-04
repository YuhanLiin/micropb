use micropb_gen::{
    Config, EncodeDecode, Generator,
    config::{CustomField, IntSize, OptionalRepr},
};

fn scoping() {
    let mut generator = Generator::new();
    generator
        .use_container_alloc()
        .configure(".", Config::new().optional_repr(OptionalRepr::Option));
    generator
        .compile_protos(
            &["proto/scoping.proto"],
            std::env::var("OUT_DIR").unwrap() + "/scoping.rs",
        )
        .unwrap();
}

fn main() {
    scoping();
}
