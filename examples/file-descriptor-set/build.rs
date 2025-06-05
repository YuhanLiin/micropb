use micropb_gen::{Config, EncodeDecode, Generator};

fn main() {
    let mut gen = Generator::new();
    gen.use_container_std()
        .encode_decode(EncodeDecode::DecodeOnly)
        .configure(
            ".",
            Config::new()
                .no_clone_impl(true)
                .no_partial_eq_impl(true)
                .minimal_accessors(true),
        )
        .compile_protos(&["google/protobuf/descriptor.proto"], "descriptor.rs")
        .unwrap();
}
