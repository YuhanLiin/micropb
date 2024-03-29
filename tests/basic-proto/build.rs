use micropb_gen::Generator;

fn main() {
    //println!("cargo::rerun-if-changed=proto/basic.proto");

    let mut generator = Generator::new();
    generator
        .compile_protos(
            &["proto/basic.proto"],
            std::env::var("OUT_DIR").unwrap() + "/proto.rs",
        )
        .unwrap();
}
