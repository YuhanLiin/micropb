use micropb_gen::{config::IntSize, Config, EncodeDecode, Generator};

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
}
