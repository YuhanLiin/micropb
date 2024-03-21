pub mod config;
pub mod generator;
mod pathtree;
mod utils;

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

pub use config::Config;
pub use generator::Generator;
use prost::Message;
use prost_types::FileDescriptorSet;

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure(&mut self, mut proto_path: &str, config: Config) {
        if proto_path.starts_with('.') {
            proto_path = &proto_path[1..];
        }

        self.config_tree
            .root
            .add_path(proto_path.split('.'))
            .value_mut()
            .get_or_insert_with(Default::default)
            .merge(&config);
    }

    fn outdir_path(&self) -> io::Result<PathBuf> {
        env::var_os("OUT_DIR")
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "OUT_DIR environment variable is not set",
                )
            })
            .map(Into::into)
    }

    pub fn compile_protos(&mut self, protos: &[impl AsRef<Path>]) -> io::Result<()> {
        let fdset_file = self.outdir_path()?.join("micropb-fdset");

        // Get protoc command from PROTOC env-var, otherwise just use "protoc"
        let mut cmd = Command::new(env::var("PROTOC").as_deref().unwrap_or("protoc"));
        cmd.arg("-o").arg(fdset_file.as_os_str());

        for proto in protos {
            cmd.arg(proto.as_ref());
        }

        let output = cmd.output()?;
        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("protoc failed: {}", String::from_utf8_lossy(&output.stderr)),
            ));
        }

        self.compile_fdset_file(fdset_file)
    }

    pub fn compile_fdset_file(&mut self, fdset_file: impl AsRef<Path>) -> io::Result<()> {
        let bytes = fs::read(fdset_file)?;
        let fdset = FileDescriptorSet::decode(&*bytes)?;
        self.compile_fdset(&fdset)
    }

    pub fn compile_fdset(&mut self, fdset: &FileDescriptorSet) -> io::Result<()> {
        for file in &fdset.file {
            self.generate_fdproto(file)?;
        }
        Ok(())
    }
}
