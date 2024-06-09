//! Placeholder

#![warn(missing_docs)]

pub mod config;
mod generator;
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
/// Whether to include encode and decode logic
pub enum EncodeDecode {
    /// Only include encode logic
    EncodeOnly,
    /// Only include decode logic
    DecodeOnly,
    #[default]
    /// Include both encode and decode logic
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
    /// Create new generator with default settings
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure(&mut self, mut proto_path: &str, config: Config) -> &mut Self {
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
        self
    }

    /// Configure the generator to generate `heapless` containers for Protobuf `string`, `bytes`,
    /// repeated, and `map` fields.
    ///
    /// Specifically, `heapless::String` is generated for `string` fields, `heapless::Vec` is
    /// generated for `bytes` and repeated fields, and `heapless::FnvIndexMap` is generated for
    /// `map` fields. This uses [`configure`](Self::configure) under the hood, so configurations
    /// set by this call can all be overriden by future configurations.
    ///
    /// # Note
    /// Since `heapless` containers are fixed size, [`max_len`] or [`max_bytes`] must be set for
    /// all fields that generate these containers.
    pub fn use_container_heapless(&mut self) -> &mut Self {
        self.configure(
            ".",
            Config::new()
                .vec_type("::micropb::heapless::Vec")
                .string_type("::micropb::heapless::String")
                .map_type("::micropb::heapless::FnvIndexMap"),
        );
        self
    }

    /// Configure the generator to generate `arrayvec` containers for Protobuf `string`, `bytes`,
    /// and repeated fields.
    ///
    /// Specifically, `arrayvec::ArrayString` is generated for `string` fields, and
    /// `arrayvec::ArrayVec` is generated for `bytes` and repeated fields. This uses
    /// [`configure`](Self::configure) under the hood, so configurations set by this call can all
    /// be overriden by future configurations.
    ///
    /// # Note
    /// No container is configured for `map` fields, since `arrayvec` doesn't have a suitable map
    /// type. If the .proto files contain `map` fields, [`map_type`] needs to be configured
    /// separately.
    ///
    /// Since `arrayvec` containers are fixed size, [`max_len`] or [`max_bytes`] must be set for
    /// all fields that generate these containers.
    pub fn use_container_arrayvec(&mut self) -> &mut Self {
        self.configure(
            ".",
            Config::new()
                .vec_type("::micropb::arrayvec::ArrayVec")
                .string_type("::micropb::arrayvec::ArrayString"),
        );
        self
    }

    /// Configure the generator to generate `alloc` containers for Protobuf `string`, `bytes`,
    /// repeated, and `map` fields.
    ///
    /// Specifically, `alloc::string::String` is generated for `string` fields, `alloc::vec::Vec`
    /// is generated for `bytes` and repeated fields, and `alloc::collections::BTreeMap` is
    /// generated for `map` fields. This uses [`configure`](Self::configure) under the hood, so
    /// configurations set by this call can all be overriden by future configurations.
    ///
    /// # Note
    /// Since `alloc` containers are dynamic size, [`max_len`] and [`max_bytes`] must NOT be set for
    /// all fields that generate these containers.
    pub fn use_container_alloc(&mut self) -> &mut Self {
        self.configure(
            ".",
            Config::new()
                .vec_type("::alloc::vec::Vec")
                .string_type("::alloc::string::String")
                .map_type("::alloc::collections::BTreeMap"),
        );
        self
    }

    /// Compile `.proto` files into a single Rust file.
    ///
    /// # Example
    /// ```
    /// // build.rs
    /// let mut gen = micropb_gen::Generator::new();
    /// gen.compile_protos(&["server.proto", "client.proto"],
    ///                     std::env::var("OUT_DIR").unwrap() + "/output.rs").unwrap();
    /// ```
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

    /// Compile a Protobuf file descriptor set into a Rust file.
    ///
    /// Similar to [`compile_protos`](Self::compile_protos), but it does not invoke `protoc` and
    /// instead takes a file descriptor set.
    pub fn compile_fdset_file(
        &mut self,
        fdset_file: impl AsRef<Path>,
        out_filename: impl AsRef<Path>,
    ) -> io::Result<()> {
        let bytes = fs::read(fdset_file)?;
        let fdset = FileDescriptorSet::decode(&*bytes)?;
        let code = self.generate_fdset(&fdset)?;

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

        let mut file = fs::File::create(out_filename)?;
        file.write_all(output.as_bytes())?;

        self.warn_unused_configs();
        Ok(())
    }

    /// Determine whether the generator strips enum names from variant names.
    ///
    /// Protobuf enums commonly include the enum name as a prefix of variant names. `micropb`
    /// strips this enum name prefix by default. Setting this to `true` prevents the prefix from
    /// being stripped.
    pub fn retain_enum_prefix(&mut self, retain_enum_prefix: bool) -> &mut Self {
        self.retain_enum_prefix = retain_enum_prefix;
        self
    }

    /// Determine whether the generator formats the output code.
    ///
    /// If the `format` feature isn't enabled, this does nothing.
    pub fn format(&mut self, format: bool) -> &mut Self {
        self.format = format;
        self
    }

    /// Determine whether to generate logic for encoding and decoding Protobuf messages.
    ///
    /// Some applications don't need to support both encoding and decoding. This setting allows
    /// either the encoding or decoding logic to be omitted from the output. By default, both
    /// encoding and decoding are included.
    ///
    /// This setting allows omitting the `encode` or `decode` feature flag from `micropb`.
    pub fn encode_decode(&mut self, encode_decode: EncodeDecode) -> &mut Self {
        self.encode_decode = encode_decode;
        self
    }

    /// When set, the file descriptor set generated by `protoc` is written to the provided path,
    /// instead of a temporary directory.
    pub fn file_descriptor_set_path<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        self.fdset_path = Some(path.into());
        self
    }

    /// Add an argument to the `protoc` invocation when compiling Protobuf files.
    pub fn add_protoc_arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.protoc_args.push(arg.as_ref().to_owned());
        self
    }

    pub fn extern_path<P1: AsRef<str>, P2: AsRef<str>>(
        &mut self,
        proto_path: P1,
        rust_path: P2,
    ) -> &mut Self {
        self.extern_paths.insert(
            proto_path.as_ref().to_owned(),
            syn::parse_str(rust_path.as_ref()).expect("failed to tokenize extern path"),
        );
        self
    }
}

fn split_pkg_name(name: &str) -> impl Iterator<Item = &str> {
    // ignore empty segments, so empty pkg name points to root node
    name.split('.').filter(|seg| !seg.is_empty())
}
