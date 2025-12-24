use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use micropb_gen::{Config, Generator};

// Generate Rust module from .proto files
fn proto_generate() {
    let mut gen = Generator::new();
    gen.use_container_heapless()
        .configure(".raw.RawMsg.payload", Config::new().max_bytes(8))
        .configure(".packet.Init.version", Config::new().max_bytes(8))
        .configure(".packet.LogBundle.logs", Config::new().max_len(4))
        .add_protoc_arg("-Iproto");
    gen.compile_protos(
        &[
            "accel.proto",
            "gps.proto",
            "packet.proto",
            "raw.proto",
            "google/protobuf/timestamp.proto",
        ],
        std::env::var("OUT_DIR").unwrap() + "/arm-example-proto.rs",
    )
    .unwrap();
    println!("cargo:rerun-if-changed=proto");
}

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");

    proto_generate();
}
