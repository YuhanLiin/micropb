use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let workspace_target_dir = out_dir
        .ancestors()
        .nth(4)
        .expect("OUT_DIR is not under a Cargo target directory");
    let profile = std::env::var("PROFILE").unwrap();
    let executable_name = if cfg!(windows) {
        "micropb-cli.exe"
    } else {
        "micropb-cli"
    };
    let cli = workspace_target_dir.join(&profile).join(executable_name);

    let output = out_dir.join("cli_test.rs");

    let status = Command::new(&cli)
        .arg("--output")
        .arg(&output)
        .arg("--container")
        .arg("std")
        .arg("--no-format")
        .arg("--encode-cache")
        .arg("--config-file")
        .arg("proto/cli.toml:cli_test")
        .arg("-Iproto")
        .arg("proto/cli.proto")
        .status()
        .unwrap_or_else(|error| {
            panic!(
                "failed to run CLI at {}: {error}; build it first with `cargo build -p micropb-cli`",
                cli.display()
            )
        });

    assert!(
        status.success(),
        "micropb-cli failed to generate fixture code"
    );
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-env-changed=CARGO_TARGET_DIR");
}
