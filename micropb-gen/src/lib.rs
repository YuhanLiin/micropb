pub mod config;
pub mod generator;
mod pathtree;
mod utils;

use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

pub use config::Config;
pub use generator::Generator;
use prost::Message;
use prost_types::FileDescriptorSet;

#[derive(Debug, Clone, Copy, Default)]
pub enum EncodeDecode {
    EncodeOnly,
    DecodeOnly,
    #[default]
    Both,
}

impl EncodeDecode {
    fn is_encode(self) -> bool {
        matches!(self, Self::EncodeOnly | Self::Both)
    }

    fn is_decode(self) -> bool {
        matches!(self, Self::DecodeOnly | Self::Both)
    }
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure(&mut self, mut proto_path: &str, config: Config) {
        if proto_path.starts_with('.') {
            proto_path = &proto_path[1..];
        }

        let config_slot = self
            .config_tree
            .root
            .add_path(split_pkg_name(proto_path))
            .value_mut();
        match config_slot {
            Some(existing) => existing.merge(&config),
            None => *config_slot = Some(Box::new(config)),
        }
    }

    pub fn use_container_heapless(&mut self) {
        self.configure(
            ".",
            Config::new()
                .vec_type("::micropb::heapless::Vec")
                .string_type("::micropb::heapless::String")
                .map_type("::micropb::heapless::FnvIndexMap"),
        );
    }

    pub fn use_container_arrayvec(&mut self) {
        self.configure(
            ".",
            Config::new()
                .vec_type("::micropb::arrayvec::ArrayVec")
                .string_type("::micropb::arrayvec::ArrayString"),
        );
    }

    pub fn use_container_alloc(&mut self) {
        self.configure(
            ".",
            Config::new()
                .vec_type("::alloc::vec::Vec")
                .string_type("::alloc::string::String")
                .map_type("::alloc::collections::BTreeMap"),
        );
    }

    pub fn compile_protos(
        &mut self,
        protos: &[impl AsRef<Path>],
        out_filename: impl AsRef<Path>,
    ) -> io::Result<()> {
        let tmp;
        let fdset_file = if let Some(fdset_path) = &self.fdset_path {
            fdset_path.to_owned()
        } else {
            tmp = tempfile::tempdir()?;
            tmp.path().join("micropb-fdset")
        };

        // Get protoc command from PROTOC env-var, otherwise just use "protoc"
        let mut cmd = Command::new(env::var("PROTOC").as_deref().unwrap_or("protoc"));
        cmd.arg("-o").arg(fdset_file.as_os_str());
        cmd.args(&self.protoc_args);

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

        self.compile_fdset_file(fdset_file, out_filename)
    }

    pub fn compile_fdset_file(
        &mut self,
        fdset_file: impl AsRef<Path>,
        out_filename: impl AsRef<Path>,
    ) -> io::Result<()> {
        let bytes = fs::read(fdset_file)?;
        let fdset = FileDescriptorSet::decode(&*bytes)?;
        let code = self.generate_fdset(&fdset);

        let mut file = fs::File::create(out_filename)?;

        #[cfg(feature = "format")]
        let output = if self.format {
            prettyplease::unparse(
                &syn::parse2(code).expect("output code should be parseable as a file"),
            )
        } else {
            code.to_string()
        };
        #[cfg(not(feature = "format"))]
        let output = code.to_string();
        file.write_all(output.as_bytes())?;
        Ok(())
    }

    pub fn retain_enum_prefix(&mut self, retain_enum_prefix: bool) -> &mut Self {
        self.retain_enum_prefix = retain_enum_prefix;
        self
    }

    pub fn format(&mut self, format: bool) -> &mut Self {
        self.format = format;
        self
    }

    pub fn use_std(&mut self, use_std: bool) -> &mut Self {
        self.use_std = use_std;
        self
    }

    pub fn encode_decode(&mut self, encode_decode: EncodeDecode) -> &mut Self {
        self.encode_decode = encode_decode;
        self
    }

    pub fn file_descriptor_set_path<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        self.fdset_path = Some(path.into());
        self
    }

    pub fn add_protoc_arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.protoc_args.push(arg.as_ref().to_owned());
        self
    }
}

fn split_pkg_name(name: &str) -> impl Iterator<Item = &str> {
    // ignore empty segments, so empty pkg name points to root node
    name.split('.').filter(|seg| !seg.is_empty())
}
