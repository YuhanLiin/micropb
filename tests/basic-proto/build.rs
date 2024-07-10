use micropb_gen::{
    config::{CustomField, IntSize, OptionalRepr},
    Config, EncodeDecode, Generator,
};

fn no_config() {
    let mut generator = Generator::new();
    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/basic3.proto",
                "proto/nested.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + "/no_config.rs",
        )
        .unwrap();
}

fn boxed_and_option() {
    let mut generator = Generator::new();
    generator.use_container_alloc();

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
    generator.configure(".nested.Nested.enumeration", Config::new().boxed(true));
    generator.configure(".nested.Nested.inner_msg", Config::new().boxed(true));
    generator.configure(".nested.Nested.InnerMsg.val", Config::new().boxed(true));

    generator.configure(".Data.s", Config::new().boxed(true));
    generator.configure(
        ".Data.b",
        Config::new()
            .boxed(true)
            .optional_repr(OptionalRepr::Hazzer),
    );
    generator.configure(".List", Config::new().boxed(true));
    generator.configure(".NumList", Config::new().boxed(true));
    generator.configure(".StrList", Config::new().boxed(true));
    generator.configure(".FixedList", Config::new().boxed(true));
    generator.configure(".Map", Config::new().boxed(true));

    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/nested.proto",
                "proto/collections.proto",
                "proto/map.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + "/boxed_and_option.rs",
        )
        .unwrap();
}

fn int_type() {
    let mut generator = Generator::new();
    generator.configure(".basic.Enum", Config::new().enum_int_size(IntSize::S8));
    generator.configure(
        ".basic.BasicTypes.int32_num",
        Config::new().int_size(IntSize::S8),
    );
    generator.configure(
        ".basic.BasicTypes.int64_num",
        Config::new().int_size(IntSize::S16),
    );
    generator.configure(
        ".basic.BasicTypes.uint32_num",
        Config::new().int_size(IntSize::S8),
    );
    generator.configure(
        ".basic.BasicTypes.uint64_num",
        Config::new().int_size(IntSize::S16),
    );
    generator.configure(
        ".basic.BasicTypes.sfixed32_num",
        Config::new().int_size(IntSize::S64),
    );
    generator.configure(
        ".basic.BasicTypes.sfixed64_num",
        Config::new().int_size(IntSize::S32),
    );
    generator.configure(
        ".basic.BasicTypes.fixed32_num",
        Config::new().int_size(IntSize::S64),
    );
    generator.configure(
        ".basic.BasicTypes.fixed64_num",
        Config::new().int_size(IntSize::S32),
    );
    generator.configure(
        ".basic.BasicTypes.sint64_num",
        Config::new().int_size(IntSize::S32),
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
    // Only the `super` field was renamed, every other field are raw identifiers
    generator.configure(
        ".crate.self.async.Msg.super",
        Config::new().rename_field("super_"),
    );

    generator
        .compile_protos(
            &["proto/keyword_fields.proto"],
            std::env::var("OUT_DIR").unwrap() + "/keyword_fields.rs",
        )
        .unwrap();
}

fn container_heapless() {
    let mut generator = Generator::new();
    generator.use_container_heapless();
    generator.configure(".Data.s", Config::new().max_bytes(3));
    generator.configure(".Data.b", Config::new().max_bytes(5));
    generator.configure(".List.list", Config::new().max_len(2));
    generator.configure(".NumList.list", Config::new().max_len(2));
    generator.configure(".NumList.list.elem", Config::new().int_size(IntSize::S8));
    generator.configure(".StrList.list", Config::new().max_len(3));
    generator.configure(".StrList.list.elem", Config::new().max_bytes(2));
    generator.configure(".FixedList.list", Config::new().max_len(2));
    generator.configure(".EnumList.list", Config::new().max_len(2));

    generator.configure(".Map.mapping", Config::new().max_len(8));
    generator.configure(".Map.mapping.key", Config::new().max_bytes(4));
    generator.configure(".Map.mapping.value", Config::new().max_bytes(3));

    generator
        .compile_protos(
            &["proto/collections.proto", "proto/map.proto"],
            std::env::var("OUT_DIR").unwrap() + "/container_heapless.rs",
        )
        .unwrap();
}

fn container_arrayvec() {
    let mut generator = Generator::new();
    generator.use_container_arrayvec();
    generator.configure(".Data.s", Config::new().max_bytes(3));
    generator.configure(".Data.b", Config::new().max_bytes(5));
    generator.configure(".List.list", Config::new().max_len(2));
    generator.configure(".NumList.list", Config::new().max_len(2));
    generator.configure(".NumList.list.elem", Config::new().int_size(IntSize::S8));
    generator.configure(".StrList.list", Config::new().max_len(3));
    generator.configure(".StrList.list.elem", Config::new().max_bytes(2));
    generator.configure(".FixedList.list", Config::new().max_len(2));
    generator.configure(".EnumList.list", Config::new().max_len(2));

    generator
        .compile_protos(
            &["proto/collections.proto"],
            std::env::var("OUT_DIR").unwrap() + "/container_arrayvec.rs",
        )
        .unwrap();
}

fn container_alloc() {
    let mut generator = Generator::new();
    generator.use_container_alloc();
    generator.configure(".NumList.list.elem", Config::new().int_size(IntSize::S8));

    generator
        .compile_protos(
            &["proto/collections.proto", "proto/map.proto"],
            std::env::var("OUT_DIR").unwrap() + "/container_alloc.rs",
        )
        .unwrap();
}

fn custom_field() {
    let mut generator = Generator::new();
    generator.configure(
        ".",
        Config::new()
            .no_debug_impl(true)
            .no_clone_impl(true)
            .no_partial_eq_impl(true),
    );
    generator.configure(
        ".nested.Nested.inner",
        Config::new()
            .custom_field(CustomField::Type(
                "crate::custom_field::MockField".to_owned(),
            ))
            .rename_field("custom_inner"),
    );
    generator.configure(
        ".nested.Nested.basic",
        Config::new().custom_field(CustomField::Delegate("custom_inner".to_owned())),
    );
    generator.configure(
        ".nested.Nested",
        Config::new().unknown_handler("crate::custom_field::MockField"),
    );

    generator.configure(
        ".List.list",
        Config::new().custom_field(CustomField::Type(
            "crate::custom_field::MockField".to_owned(),
        )),
    );
    generator.configure(".Data", Config::new().skip(true));
    generator.configure(".NumList", Config::new().skip(true));
    generator.configure(".StrList", Config::new().skip(true));
    generator.configure(".FixedList", Config::new().skip(true));
    generator.configure(".EnumList", Config::new().skip(true));

    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/nested.proto",
                "proto/collections.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + "/custom_field.rs",
        )
        .unwrap();
}

fn implicit_presence() {
    let mut generator = Generator::new();
    generator.use_container_alloc();
    generator
        .compile_protos(
            &["proto/implicit_presence.proto"],
            std::env::var("OUT_DIR").unwrap() + "/implicit_presence.rs",
        )
        .unwrap();
}

fn extern_import() {
    let mut gen1 = Generator::new();
    gen1.compile_protos(
        &["proto/basic.proto"],
        std::env::var("OUT_DIR").unwrap() + "/import_basic.rs",
    )
    .unwrap();

    let mut gen2 = Generator::new();
    // Replace `BasicTypes` with an empty message
    gen2.extern_type_path(".basic.BasicTypes", "crate::extern_import::Empty")
        // Replace `Enum` with the generated enum type
        .extern_type_path(".basic.Enum", "crate::extern_import::proto::basic::Enum")
        .compile_protos(
            &["proto/nested.proto"],
            std::env::var("OUT_DIR").unwrap() + "/import_nested.rs",
        )
        .unwrap();
}

fn lifetime_fields() {
    let mut generator = Generator::new();
    generator.encode_decode(EncodeDecode::EncodeOnly);
    generator.configure(".", Config::new().no_debug_impl(true).no_default_impl(true));
    generator.configure(
        ".nested.Nested.inner",
        Config::new().custom_field(CustomField::Type(
            "crate::lifetime_fields::RefField<'a>".to_owned(),
        )),
    );
    generator.configure(
        ".nested.Nested.basic",
        Config::new().custom_field(CustomField::Delegate("inner".to_owned())),
    );
    generator.configure(
        ".nested.Nested.InnerMsg",
        Config::new().unknown_handler("Option<crate::lifetime_fields::RefField<'a>>"),
    );
    generator.configure(
        ".basic.BasicTypes.int32_num",
        Config::new().custom_field(CustomField::Type(
            "crate::lifetime_fields::RefField<'a>".to_owned(),
        )),
    );
    generator
        .compile_protos(
            &["proto/basic.proto", "proto/nested.proto"],
            std::env::var("OUT_DIR").unwrap() + "/lifetime_fields.rs",
        )
        .unwrap();
}

fn recursive() {
    let mut generator = Generator::new();
    generator.configure(".Recursive.recursive", Config::new().boxed(true));
    generator.configure(".Recursive.of", Config::new().boxed(true));
    generator.configure(".Recursive.rec", Config::new().boxed(true));
    generator
        .compile_protos(
            &["proto/recursive.proto"],
            std::env::var("OUT_DIR").unwrap() + "/recursive.rs",
        )
        .unwrap();
}

fn conflicting_names() {
    let mut generator = Generator::new();
    generator
        .compile_protos(
            &["proto/conflicting_names.proto"],
            std::env::var("OUT_DIR").unwrap() + "/conflicting_names.rs",
        )
        .unwrap();
}

fn main() {
    no_config();
    boxed_and_option();
    int_type();
    skip();
    keyword_fields();
    container_heapless();
    container_arrayvec();
    container_alloc();
    custom_field();
    implicit_presence();
    extern_import();
    lifetime_fields();
    recursive();
    conflicting_names();
}
