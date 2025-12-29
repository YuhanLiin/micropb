use micropb_gen::{
    config::{CustomField, IntSize},
    Config, EncodeDecode, Generator,
};

fn main() {
    let mut gen = Generator::new();
    // Only generate encode logic
    gen.encode_decode(EncodeDecode::EncodeOnly)
        // Set all int sizes to 32 bits, since 64-bit support isn't enabled on micropb
        .configure(".", Config::new().int_size(IntSize::S32))
        .add_protoc_arg("-I..");
    gen.compile_protos(
        &["example.proto"],
        std::env::var("OUT_DIR").unwrap() + "/encode_only.rs",
    )
    .unwrap();

    let mut gen = Generator::new();
    // Only generate encode logic
    gen.encode_decode(EncodeDecode::EncodeOnly)
        .encode_cache(true)
        .single_oneof_msg_as_enum(true)
        .use_container_heapless()
        .configure(".", Config::new().type_attributes("#[derive(Eq)]"))
        .configure(".Many", Config::new().max_len(4))
        .configure(".Lookup", Config::new().max_len(2))
        .configure(
            ".Leaf.field",
            Config::new().custom_field(CustomField::Type(
                "crate::deeply_nested::Counter".to_owned(),
            )),
        );
    gen.compile_protos(
        &["deeply_nested.proto"],
        std::env::var("OUT_DIR").unwrap() + "/deeply_nested.rs",
    )
    .unwrap();
}
