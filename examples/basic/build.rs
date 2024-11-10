fn main() {
    let mut gen = micropb_gen::Generator::new();
    // Compile example.proto into a Rust module
    gen.compile_protos(
        &["example.proto"],
        std::env::var("OUT_DIR").unwrap() + "/example.rs",
    )
    .unwrap();
}
