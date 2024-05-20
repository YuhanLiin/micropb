use micropb_gen::{Config, Generator};

fn main() {
    // Enable serde and Eq derives on the proto type
    let mut generator = Generator::new();
    generator
        .use_container_heapless()
        .configure(
            ".Data",
            Config::new()
                .type_attributes("#[derive(serde::Serialize, serde::Deserialize)] #[derive(Eq)]")
                .max_bytes(4)
                .max_len(2),
        )
        // Don't enable Eq on the enum, which already implements Eq by default
        .configure(
            ".Data.Enum",
            Config::new().type_attributes("#[derive(serde::Serialize, serde::Deserialize)]"),
        )
        .configure(
            ".Data.int",
            Config::new().field_attributes("#[serde(default)]"),
        )
        .configure(
            ".Data.s",
            Config::new().field_attributes("#[serde(default)]"),
        )
        .configure(
            ".Data.b",
            Config::new().field_attributes("#[serde(default)]"),
        )
        .configure(
            ".Data.list",
            Config::new().field_attributes("#[serde(default)]"),
        );

    generator
        .compile_protos(
            &["proto/data.proto"],
            std::env::var("OUT_DIR").unwrap() + "/serde_proto.rs",
        )
        .unwrap();
}
