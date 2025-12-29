use std::fmt::Arguments;

use micropb_gen::{Config, Generator, config::CustomField};

use tempfile::NamedTempFile;

// If we get any warnings, fail the test
fn warn_panic(args: Arguments) {
    panic!("Unexpected warning: {args}");
}

fn compile(generator: Generator) -> String {
    let file = NamedTempFile::new().unwrap();
    let err = generator
        .compile_protos(&["tests/test.proto"], file.path())
        .unwrap_err();
    err.to_string()
}

#[test]
fn no_string() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.configure(
        ".",
        Config::new()
            .vec_type("Vec")
            .map_type("HashMap")
            .bytes_type("Bytes"),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("string_type not configured"));
}

#[test]
fn no_bytes() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.configure(".", Config::new().string_type("String").map_type("HashMap"));
    generator.configure(".test.Msg.list", Config::new().vec_type("Vec"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.bt)"));
    assert!(err.contains("bytes_type not configured"));
}

#[test]
fn no_vec_repeated() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.configure(
        ".",
        Config::new()
            .string_type("String")
            .map_type("HashMap")
            .bytes_type("Bytes"),
    );
    generator.configure(".test.Msg.bt", Config::new().vec_type("Vec"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.list)"));
    assert!(err.contains("vec_type not configured"));
}

#[test]
fn no_map() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.configure(
        ".",
        Config::new()
            .string_type("String")
            .vec_type("Vec")
            .bytes_type("Bytes"),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("map_type not configured"));
}

#[test]
fn long_default_string() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_heapless();
    generator.configure(".", Config::new().max_len(4));
    generator.configure(".test.Msg.bt", Config::new().max_bytes(4));
    generator.configure(".test.Msg.st", Config::new().max_bytes(2));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("limited to 2 bytes"));
    assert!(err.contains("default value is 3 bytes"));
}

#[test]
fn long_default_bytes() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_heapless();
    generator.configure(".", Config::new().max_len(4));
    generator.configure(".test.Msg.bt", Config::new().max_bytes(2));
    generator.configure(".test.Msg.st", Config::new().max_bytes(4));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.bt)"));
    assert!(err.contains("limited to 2 bytes"));
    assert!(err.contains("default value is 3 bytes"));
}

#[test]
fn parse_string_type() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.configure(
        ".",
        Config::new()
            .string_type("String::")
            .bytes_type("Bytes")
            .vec_type("Vec")
            .map_type("HashMap"),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("Failed to parse string or bytes type"));
}

#[test]
fn parse_bytes_type() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.configure(
        ".",
        Config::new()
            .string_type("String")
            .bytes_type("Bytes::")
            .vec_type("Vec")
            .map_type("HashMap"),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.bt)"));
    assert!(err.contains("Failed to parse string or bytes"));
}

#[test]
fn parse_vec_type() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.configure(
        ".",
        Config::new()
            .string_type("String")
            .bytes_type("Bytes")
            .vec_type("Vec::")
            .map_type("HashMap"),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.list)"));
    assert!(err.contains("Failed to parse vec_type"));
}

#[test]
fn parse_map_type() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.configure(
        ".",
        Config::new()
            .string_type("String")
            .bytes_type("Bytes")
            .vec_type("Vec")
            .map_type("HashMap::"),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("Failed to parse map_type"));
}

#[test]
fn parse_rename() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg.kv", Config::new().rename_field("two words"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("Failed to parse rename_field"));
}

#[test]
fn parse_type_attr() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg.of", Config::new().type_attributes(""));
    generator.configure(".test.Msg._has", Config::new().type_attributes(""));
    generator.configure(".test.Msg", Config::new().type_attributes("error"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg)"));
    assert!(err.contains("Failed to parse type_attributes"));
}

#[test]
fn parse_type_attr_hazzer() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg._has", Config::new().type_attributes("error"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg._has)"));
    assert!(err.contains("Failed to parse type_attributes"));
}

#[test]
fn parse_type_attr_enum() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Enum", Config::new().type_attributes("error"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Enum)"));
    assert!(err.contains("Failed to parse type_attributes"));
}

#[test]
fn parse_type_attr_oneof() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg.of", Config::new().type_attributes("error"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.of)"));
    assert!(err.contains("Failed to parse type_attributes"));
}

#[test]
fn parse_field_attr() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg.kv", Config::new().field_attributes("error"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("Failed to parse field_attributes"));
}

#[test]
fn parse_field_attr_hazzer() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg._has", Config::new().field_attributes("error"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg._has)"));
    assert!(err.contains("Failed to parse field_attributes"));
}

#[test]
fn parse_field_attr_unknown() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg", Config::new().unknown_handler("Handler"));
    generator.configure(
        ".test.Msg._unknown",
        Config::new().field_attributes("error"),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg._unknown)"));
    assert!(err.contains("Failed to parse field_attributes"));
}

#[test]
fn parse_field_attr_oneof() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg.of", Config::new().field_attributes("error"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.of)"));
    assert!(err.contains("Failed to parse field_attributes"));
}

#[test]
fn parse_unknown_handler() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(".test.Msg", Config::new().unknown_handler("Type<"));
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg)"));
    assert!(err.contains("Failed to parse unknown_handler"));
}

#[test]
fn parse_custom_type() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(
        ".test.Msg.kv",
        Config::new().custom_field(CustomField::Type("Type<".to_owned())),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("Failed to parse custom field"));
}

#[test]
fn parse_custom_type_oneof() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(
        ".test.Msg.of",
        Config::new().custom_field(CustomField::Type("Type<".to_owned())),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.of)"));
    assert!(err.contains("Failed to parse custom field"));
}

#[test]
fn parse_custom_delegate() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(
        ".test.Msg.st",
        Config::new().custom_field(CustomField::Delegate("Type<>".to_owned())),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("Failed to parse custom delegate"));
}

#[test]
fn parse_custom_type_delegate() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    generator.use_container_alloc();
    generator.configure(
        ".test.Msg.of",
        Config::new().custom_field(CustomField::Delegate("Type<".to_owned())),
    );
    let err = compile(generator);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.of)"));
    assert!(err.contains("Failed to parse custom delegate"));
}

#[test]
#[should_panic = "Unused configuration path: \".Msg\""]
fn warn_unused_config() {
    let mut generator = Generator::with_warning_callback(warn_panic);
    // Warnings are only emitted if code generation succeeds, which is ensured by using alloc
    generator.use_container_alloc();
    // Unused config path as Msg
    generator.configure(".Msg", Config::new().max_len(5));
    compile(generator);
}
