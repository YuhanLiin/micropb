use micropb_gen::Generator;

fn main() {
    let mut gen = Generator::new();
    gen.use_container_std();
    gen.compile_protos(
        &["test.proto"],
        // We want to see the output file, so don't generate it in OUT_DIR
        "test.proto.rs",
    )
    .unwrap();
}
