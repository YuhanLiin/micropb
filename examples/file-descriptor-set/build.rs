use micropb_gen::{EncodeDecode, Generator};

fn main() {
    let mut gen = Generator::new();
    gen.use_container_std()
        .encode_decode(EncodeDecode::DecodeOnly)
        .compile_protos(&["google/protobuf/descriptor.proto"], "descriptor.rs")
        .unwrap();
}
