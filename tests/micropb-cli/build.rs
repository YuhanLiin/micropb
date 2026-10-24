use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let cli_target_dir = manifest_dir.join("../../micropb-cli/target/debug");
    let executable_name = if cfg!(windows) {
        "micropb-cli.exe"
    } else {
        "micropb-cli"
    };
    let cli = cli_target_dir.join(executable_name);

    let output = out_dir.join("cli_test.rs");
    let proto = PathBuf::from("proto/cli.proto");
    let toml = PathBuf::from("proto/cli.toml");

    let status = Command::new(&cli)
        .arg("--output")
        .arg(&output)
        .arg("--container")
        .arg("std")
        .arg("--no-format")
        .arg("--encode-cache")
        .arg("--config-file")
        .arg(format!("{}:cli_test", toml.display()))
        .arg("--retain-enum-prefix")
        .arg("-Iproto")
        .arg("--no-suffixed-package-names")
        .arg(&proto)
        .status()
        .unwrap_or_else(|error| panic!("failed to run CLI at {}: {error}", cli.display()));

    assert!(
        status.success(),
        "micropb-cli failed to generate fixture code"
    );
    println!("cargo:rerun-if-changed={}", cli.display());
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-env-changed=MICROPB_CLI_TARGET_DIR");
}
