use std::fmt::Arguments;

use micropb_gen::{config::CustomField, Config, Generator};

use tempfile::NamedTempFile;

// If we get any warnings, fail the test
fn warn_panic(args: Arguments) {
    panic!("Unexpected warning: {args}");
}

fn compile(mut gen: Generator) -> String {
    let file = NamedTempFile::new().unwrap();
    let err = gen
        .compile_protos(&["tests/test.proto"], file.path())
        .unwrap_err();
    err.into_inner().unwrap().to_string()
}

#[test]
fn no_string() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.configure(".", Config::new().vec_type("Vec").map_type("HashMap"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("string_type was not configured"));
}

#[test]
fn no_vec_bytes() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.configure(".", Config::new().string_type("String").map_type("HashMap"));
    gen.configure(".test.Msg.list", Config::new().vec_type("Vec"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.bt)"));
    assert!(err.contains("vec_type was not configured"));
}

#[test]
fn no_vec_repeated() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.configure(".", Config::new().string_type("String").map_type("HashMap"));
    gen.configure(".test.Msg.bt", Config::new().vec_type("Vec"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.list)"));
    assert!(err.contains("vec_type was not configured"));
}

#[test]
fn no_map() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.configure(".", Config::new().string_type("String").vec_type("Vec"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("map_type was not configured"));
}

#[test]
fn long_default_string() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_heapless();
    gen.configure(".test.Msg.st", Config::new().max_bytes(2));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("limited to 2 bytes"));
    assert!(err.contains("default value is 3 bytes"));
}

#[test]
fn long_default_bytes() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_heapless();
    gen.configure(".test.Msg.bt", Config::new().max_bytes(2));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.bt)"));
    assert!(err.contains("limited to 2 bytes"));
    assert!(err.contains("default value is 3 bytes"));
}

#[test]
fn parse_string_type() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.configure(
        ".",
        Config::new()
            .string_type("String::")
            .vec_type("Vec")
            .map_type("HashMap"),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("Failed to parse string_type"));
}

#[test]
fn parse_vec_type() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.configure(
        ".",
        Config::new()
            .string_type("String")
            .vec_type("Vec::")
            .map_type("HashMap"),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.bt)"));
    assert!(err.contains("Failed to parse vec_type"));
}

#[test]
fn parse_map_type() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.configure(
        ".",
        Config::new()
            .string_type("String")
            .vec_type("Vec")
            .map_type("HashMap::"),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("Failed to parse map_type"));
}

#[test]
fn parse_rename() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg.kv", Config::new().rename_field("two words"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("Failed to parse rename_field"));
}

#[test]
fn parse_type_attr() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg.of", Config::new().type_attributes(""));
    gen.configure(".test.Msg._has", Config::new().type_attributes(""));
    gen.configure(".test.Msg", Config::new().type_attributes("error"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg)"));
    assert!(err.contains("Failed to parse type_attributes"));
}

#[test]
fn parse_type_attr_hazzer() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg._has", Config::new().type_attributes("error"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg._has)"));
    assert!(err.contains("Failed to parse type_attributes"));
}

#[test]
fn parse_type_attr_enum() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Enum", Config::new().type_attributes("error"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Enum)"));
    assert!(err.contains("Failed to parse type_attributes"));
}

#[test]
fn parse_type_attr_oneof() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg.of", Config::new().type_attributes("error"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.of)"));
    assert!(err.contains("Failed to parse type_attributes"));
}

#[test]
fn parse_field_attr() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg.kv", Config::new().field_attributes("error"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("Failed to parse field_attributes"));
}

#[test]
fn parse_field_attr_hazzer() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg._has", Config::new().field_attributes("error"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg._has)"));
    assert!(err.contains("Failed to parse field_attributes"));
}

#[test]
fn parse_field_attr_unknown() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg", Config::new().unknown_handler("Handler"));
    gen.configure(
        ".test.Msg._unknown",
        Config::new().field_attributes("error"),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg._unknown)"));
    assert!(err.contains("Failed to parse field_attributes"));
}

#[test]
fn parse_field_attr_oneof() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg.of", Config::new().field_attributes("error"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.of)"));
    assert!(err.contains("Failed to parse field_attributes"));
}

#[test]
fn parse_unknown_handler() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(".test.Msg", Config::new().unknown_handler("Type<"));
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg)"));
    assert!(err.contains("Failed to parse unknown_handler"));
}

#[test]
fn parse_custom_type() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(
        ".test.Msg.kv",
        Config::new().custom_field(CustomField::Type("Type<".to_owned())),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.kv)"));
    assert!(err.contains("Failed to parse custom field"));
}

#[test]
fn parse_custom_type_oneof() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(
        ".test.Msg.of",
        Config::new().custom_field(CustomField::Type("Type<".to_owned())),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.of)"));
    assert!(err.contains("Failed to parse custom field"));
}

#[test]
fn parse_custom_delegate() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(
        ".test.Msg.st",
        Config::new().custom_field(CustomField::Delegate("Type<>".to_owned())),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("Failed to parse custom delegate"));
}

#[test]
fn parse_custom_type_delegate() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(
        ".test.Msg.of",
        Config::new().custom_field(CustomField::Delegate("Type<".to_owned())),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.of)"));
    assert!(err.contains("Failed to parse custom delegate"));
}

#[test]
fn unfound_delegate() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    gen.use_container_alloc();
    gen.configure(
        ".test.Msg.st",
        Config::new().custom_field(CustomField::Delegate("kv".to_owned())),
    );
    let err = compile(gen);
    dbg!(&err);
    assert!(err.contains("(.test.Msg.st)"));
    assert!(err.contains("Delegate field refers to custom field of kv"));
}

#[test]
#[should_panic = "Unused configuration path: \".Msg\""]
fn warn_unused_config() {
    let mut gen = Generator::with_warning_callback(warn_panic);
    // Warnings are only emitted if code generation succeeds, which is ensured by using alloc
    gen.use_container_alloc();
    // Unused config path as Msg
    gen.configure(".Msg", Config::new().max_len(5));
    compile(gen);
}
