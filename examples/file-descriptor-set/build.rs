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
        // Override minimal accessors setting for specific paths, since the generator calls `set_`
        // APIs on specific messages
        .configure_many(
            &[
                ".google.protobuf.DescriptorProto",
                ".google.protobuf.FieldDescriptorProto",
                ".google.protobuf.FieldOptions",
                ".google.protobuf.OneofDescriptorProto",
                ".google.protobuf.MessageOptions",
                ".google.protobuf.EnumValueDescriptorProto",
            ],
            Config::new().minimal_accessors(false),
        )
        .compile_protos(&["google/protobuf/descriptor.proto"], "descriptor.rs")
        .unwrap();
}
