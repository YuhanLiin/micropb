use micropb_gen::{config::OptionalRepr, Config, Generator};

fn no_config() {
    let mut generator = Generator::new();
    generator
        .compile_protos(
            &["proto/basic.proto"],
            std::env::var("OUT_DIR").unwrap() + "/no_config.rs",
        )
        .unwrap();
}

fn boxed_and_option() {
    let mut generator = Generator::new();
    generator.configure(".BasicTypes.boolean", Config::new().boxed(true));
    generator.configure(
        ".BasicTypes.int32_num",
        Config::new().optional_repr(OptionalRepr::Option),
    );
    generator.configure(
        ".BasicTypes.uint32_num",
        Config::new()
            .boxed(false)
            .optional_repr(OptionalRepr::Hazzer),
    );
    generator
        .compile_protos(
            &["proto/basic.proto"],
            std::env::var("OUT_DIR").unwrap() + "/boxed_and_option.rs",
        )
        .unwrap();
}

fn main() {
    no_config();
    boxed_and_option();
}
