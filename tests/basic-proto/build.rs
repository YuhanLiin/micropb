use micropb_gen::{
    config::{IntType, OptionalRepr},
    Config, Generator,
};

fn no_config() {
    let mut generator = Generator::new();
    generator
        .compile_protos(
            &["proto/basic.proto", "proto/nested.proto"],
            std::env::var("OUT_DIR").unwrap() + "/no_config.rs",
        )
        .unwrap();
}

fn boxed_and_option() {
    let mut generator = Generator::new();
    generator.configure(".basic.BasicTypes.boolean", Config::new().boxed(true));
    generator.configure(
        ".basic.BasicTypes.int32_num",
        Config::new().optional_repr(OptionalRepr::Option),
    );
    generator.configure(
        ".basic.BasicTypes.uint32_num",
        Config::new()
            .boxed(true)
            .optional_repr(OptionalRepr::Hazzer),
    );

    generator
        .compile_protos(
            &["proto/basic.proto"],
            std::env::var("OUT_DIR").unwrap() + "/boxed_and_option.rs",
        )
        .unwrap();
}

fn int_type() {
    let mut generator = Generator::new();
    generator.configure(".basic.Enum", Config::new().enum_int_type(IntType::I8));
    generator.configure(
        ".basic.BasicTypes.int32_num",
        Config::new().int_type(IntType::I16),
    );

    generator
        .compile_protos(
            &["proto/basic.proto"],
            std::env::var("OUT_DIR").unwrap() + "/int_type.rs",
        )
        .unwrap();
}

fn skip() {
    let mut generator = Generator::new();
    generator.configure(".basic.Enum", Config::new().skip(true));
    generator.configure(".basic.BasicTypes", Config::new().skip(true));
    generator.configure(".nested.Nested.basic", Config::new().skip(true));
    generator.configure(".nested.Nested.inner_msg", Config::new().skip(true));
    generator.configure(".nested.Nested.inner_enum", Config::new().skip(true));
    generator.configure(".nested.Nested.enumeration", Config::new().skip(true));
    // only .nested.Nested.scalar is not skipped

    generator
        .compile_protos(
            &["proto/basic.proto", "proto/nested.proto"],
            std::env::var("OUT_DIR").unwrap() + "/skip.rs",
        )
        .unwrap();
}

fn keyword_fields() {
    let mut generator = Generator::new();
    generator.configure(".Msg.super", Config::new().rename_field("super_"));
    generator.configure(".Msg.i32", Config::new().rename_field("i32_"));
    generator.configure(".Msg.type", Config::new().rename_field("typ"));

    generator
        .compile_protos(
            &["proto/keyword_fields.proto"],
            std::env::var("OUT_DIR").unwrap() + "/keyword_fields.rs",
        )
        .unwrap();
}

fn main() {
    no_config();
    boxed_and_option();
    int_type();
    skip();
    keyword_fields();
}
