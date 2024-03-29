use micropb_gen::Generator;

fn main() {
    //println!("cargo::rerun-if-changed=proto/basic.proto");

    let mut generator = Generator::new();
    generator
        .compile_protos(&["proto/basic.proto"], "proto.rs")
        .unwrap();
}
