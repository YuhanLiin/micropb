use std::path::Path;

use micropb_gen::{
    config::{CustomField, IntSize, OptionalRepr},
    Config, EncodeDecode, Generator,
};

fn no_config(encode_cache: bool) {
    let mut generator = Generator::new();
    generator.encode_cache(encode_cache);
    let filename = if encode_cache {
        "/no_config.cached.rs"
    } else {
        "/no_config.rs"
    };
    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/basic3.proto",
                "proto/nested.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + filename,
        )
        .unwrap();
}

fn boxed_and_option(encode_cache: bool) {
    let mut generator = Generator::new();
    generator.use_container_alloc().encode_cache(encode_cache);

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
    generator.configure(
        ".basic.BasicTypes.flt",
        Config::new().boxed(true).optional_repr(OptionalRepr::None),
    );
    generator.configure(
        ".nested.Nested.basic",
        Config::new().optional_repr(OptionalRepr::None),
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

    let filename = if encode_cache {
        "/boxed_and_option.cached.rs"
    } else {
        "/boxed_and_option.rs"
    };
    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/nested.proto",
                "proto/collections.proto",
                "proto/map.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + filename,
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
    generator.configure(
        ".basic.Enum2",
        Config::new()
            .enum_unsigned(true)
            .enum_int_size(IntSize::S16),
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
    let generator = Generator::new();
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

fn container_alloc(encode_cache: bool) {
    let mut generator = Generator::new();
    generator.use_container_alloc().encode_cache(encode_cache);
    generator.configure(".NumList.list.elem", Config::new().int_size(IntSize::S8));

    let filename = if encode_cache {
        "/container_alloc.cached.rs"
    } else {
        "/container_alloc.rs"
    };
    generator
        .compile_protos(
            &["proto/collections.proto", "proto/map.proto"],
            std::env::var("OUT_DIR").unwrap() + filename,
        )
        .unwrap();
}

fn container_cow(encode_cache: bool) {
    let mut generator = Generator::new();
    generator.encode_cache(encode_cache);
    generator.configure(
        ".",
        Config::new()
            .string_type("::alloc::borrow::Cow<'a, str>")
            .vec_type("::alloc::borrow::Cow<'a, [$T]>")
            .bytes_type("::alloc::borrow::Cow<'a, [u8]>")
            .cache_vec_type("::alloc::vec::Vec<$T>"),
    );

    let filename = if encode_cache {
        "/container_cow.cached.rs"
    } else {
        "/container_cow.rs"
    };
    generator
        .compile_protos(
            &["proto/collections.proto"],
            std::env::var("OUT_DIR").unwrap() + filename,
        )
        .unwrap();
}

fn fixed_string_and_bytes(encode_cache: bool) {
    let mut generator = Generator::new();
    generator.use_container_alloc().encode_cache(encode_cache);
    generator.configure(
        ".Data.s",
        Config::new()
            .string_type("::micropb::FixedLenString<$N>")
            .max_bytes(3),
    );
    generator.configure(".Data.b", Config::new().bytes_type("[u8; $N]").max_bytes(2));

    let filename = if encode_cache {
        "/fixed_string_and_bytes.cached.rs"
    } else {
        "/fixed_string_and_bytes.rs"
    };
    generator
        .compile_protos(
            &["proto/collections.proto"],
            std::env::var("OUT_DIR").unwrap() + filename,
        )
        .unwrap();
}

fn custom_field(encode_cache: bool) {
    let mut generator = Generator::new();
    generator.encode_cache(encode_cache);
    generator.configure(".", Config::new().no_debug_impl(true).no_clone_impl(true));
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

    let filename = if encode_cache {
        "/custom_field.cached.rs"
    } else {
        "/custom_field.rs"
    };
    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/nested.proto",
                "proto/collections.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + filename,
        )
        .unwrap();
}

fn implicit_presence(encode_cache: bool) {
    let mut generator = Generator::new();
    generator.use_container_alloc().encode_cache(encode_cache);
    let filename = if encode_cache {
        "/implicit_presence.cached.rs"
    } else {
        "/implicit_presence.rs"
    };
    generator
        .compile_protos(
            &["proto/implicit_presence.proto"],
            std::env::var("OUT_DIR").unwrap() + filename,
        )
        .unwrap();
}

fn extern_import(encode_cache: bool) {
    let gen1 = Generator::new();
    gen1.compile_protos(
        &["proto/basic.proto"],
        std::env::var("OUT_DIR").unwrap() + "/import_basic.rs",
    )
    .unwrap();

    let mut gen2 = Generator::new();
    // Replace `BasicTypes` with an empty message
    gen2.extern_type_path(".basic.BasicTypes", "crate::extern_import::Empty")
        // Replace `Enum` with the generated enum type
        .extern_type_path(".basic.Enum", "crate::extern_import::proto::basic_::Enum")
        .encode_cache(encode_cache)
        .cache_extern_types(false);
    let filename = if encode_cache {
        "/import_nested.cached.rs"
    } else {
        "/import_nested.rs"
    };
    gen2.compile_protos(
        &["proto/nested.proto"],
        std::env::var("OUT_DIR").unwrap() + filename,
    )
    .unwrap();
}

fn lifetime_fields() {
    let mut generator = Generator::new();
    generator.encode_decode(EncodeDecode::EncodeOnly);
    generator.configure(".", Config::new().no_debug_impl(true).no_default_impl(true));
    // InnerMsg has a lifetime param, so we expect it to propagate everywhere
    generator.configure(
        ".nested.Nested.InnerMsg",
        Config::new().unknown_handler("Option<crate::lifetime_fields::RefField<'a>>"),
    );
    generator.configure(".nested.Nested.basic", Config::new().skip(true));

    // Configurations for collections.proto
    // The lifetimes should propagate to the correct types
    generator.configure(
        ".",
        Config::new()
            .string_type("&'a str")
            .bytes_type("&'a [u8]")
            .vec_type("&'a [$T]")
            .map_type("&'a std::collections::HashMap<$K, $V>"),
    );

    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/nested.proto",
                "proto/collections.proto",
                "proto/map.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + "/lifetime_fields.rs",
        )
        .unwrap();
}

fn static_lifetime_fields() {
    let mut generator = Generator::new();
    generator.encode_decode(EncodeDecode::EncodeOnly);
    generator.configure(
        ".",
        Config::new()
            .no_default_impl(true)
            .string_type("&'static str")
            .bytes_type("&'static [u8]")
            .vec_type("&'static [$T]")
            .map_type("&'static std::collections::HashMap<$K, $V>"),
    );

    generator
        .compile_protos(
            &["proto/collections.proto", "proto/map.proto"],
            std::env::var("OUT_DIR").unwrap() + "/static_lifetime_fields.rs",
        )
        .unwrap();
}

fn recursive() {
    let mut generator = Generator::new();
    generator.use_container_std();
    generator
        .configure(".", Config::new().optional_repr(OptionalRepr::Option))
        // Should remove Debug, Clone, and PartialEq from both the message and the oneof types
        .configure(
            ".Recursive",
            Config::new()
                .no_debug_impl(true)
                .no_clone_impl(true)
                .no_partial_eq_impl(true),
        );
    // Should work without any extra configuration
    generator
        .compile_protos(
            &["proto/recursive.proto"],
            std::env::var("OUT_DIR").unwrap() + "/recursive.rs",
        )
        .unwrap();
}

fn conflicting_names() {
    let generator = Generator::new();
    generator
        .compile_protos(
            &["proto/conflicting_names.proto"],
            std::env::var("OUT_DIR").unwrap() + "/conflicting_names.rs",
        )
        .unwrap();
}

fn default_str_escape() {
    let mut generator = Generator::new();
    generator.use_container_alloc();
    generator
        .compile_protos(
            &["proto/default_str_escape.proto"],
            std::env::var("OUT_DIR").unwrap() + "/default_str_escape.rs",
        )
        .unwrap();
}

fn extension() {
    let generator = Generator::new();
    generator
        .compile_protos(
            &["proto/extension.proto"],
            std::env::var("OUT_DIR").unwrap() + "/extension.rs",
        )
        .unwrap();
}

fn files_with_same_package() {
    let generator = Generator::new();
    generator
        .compile_protos(
            &["proto/basic.proto", "proto/basic-dup.proto"],
            std::env::var("OUT_DIR").unwrap() + "/files_with_same_package.rs",
        )
        .unwrap();
}

fn large_field_nums() {
    let generator = Generator::new();
    generator
        .compile_protos(
            &["proto/large_field_nums.proto"],
            std::env::var("OUT_DIR").unwrap() + "/large_field_nums.rs",
        )
        .unwrap();
}

fn minimal_accessors() {
    let mut generator = Generator::new();
    generator.calculate_max_size(false);
    generator.configure(".", Config::new().no_accessors(true));
    // Test what happens when there's a message that doesn't use hazzers
    generator.configure(
        ".nested.Nested",
        Config::new().optional_repr(OptionalRepr::Option),
    );
    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/basic3.proto",
                "proto/nested.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + "/minimal_accessors.rs",
        )
        .unwrap();
}

fn with_config_file() {
    let mut generator = Generator::new();
    generator.use_container_heapless();
    generator
        .parse_config_file(Path::new("proto/collections.toml"), ".")
        .unwrap();
    generator
        .parse_config_file(Path::new("proto/map.toml"), ".")
        .unwrap();
    generator
        .parse_config_file(Path::new("proto/basic.toml"), ".basic")
        .unwrap();

    generator
        .compile_protos(
            &[
                "proto/collections.proto",
                "proto/map.proto",
                "proto/basic.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + "/with_config_file.rs",
        )
        .unwrap();
}

fn single_oneof(encode_cache: bool) {
    let mut generator = Generator::new();
    generator
        .single_oneof_msg_as_enum(true)
        .encode_cache(encode_cache);
    generator.configure(".SingleOneof.inner_msg", Config::new().boxed(true));
    generator.configure(".SingleOneof.scalar", Config::new().skip(true));

    let filename = if encode_cache {
        "/single_oneof.cached.rs"
    } else {
        "/single_oneof.rs"
    };
    generator
        .compile_protos(
            &[
                "proto/basic.proto",
                "proto/nested.proto",
                "proto/single_oneof.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + filename,
        )
        .unwrap();
}

fn main() {
    no_config(true);
    no_config(false);
    boxed_and_option(true);
    boxed_and_option(false);
    int_type();
    skip();
    keyword_fields();
    container_heapless();
    container_arrayvec();
    container_alloc(true);
    container_alloc(false);
    container_cow(true);
    container_cow(false);
    custom_field(true);
    custom_field(false);
    implicit_presence(true);
    implicit_presence(false);
    extern_import(true);
    extern_import(false);
    lifetime_fields();
    static_lifetime_fields();
    recursive();
    conflicting_names();
    default_str_escape();
    extension();
    files_with_same_package();
    fixed_string_and_bytes(true);
    fixed_string_and_bytes(false);
    large_field_nums();
    minimal_accessors();
    with_config_file();
    single_oneof(true);
    single_oneof(false);
}
