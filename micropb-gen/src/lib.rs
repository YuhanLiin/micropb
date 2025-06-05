#![warn(missing_docs)]
//! `micropb-gen` compiles `.proto` files into Rust code. It is intended to be used inside
//! `build.rs` for build-time code generation.
//!
//! Unlike other Protobuf code generators in the Rust ecosystem, `micropb` is aimed for constrained
//! environments without an allocator.
//!
//! The entry point of this crate is the [`Generator`] type.
//!
//! For info on the "library layer" of `micropb-gen`, see [`micropb`].
//!
//! # Getting Started
//!
//! Add `micropb` crates to your `Cargo.toml`:
//! ```protobuf
//! [dependencies]
//! # Allow types from `heapless` to be used for container fields
//! micropb = { version = "0.2.0", features = ["container-heapless"] }
//!
//! [build-dependencies]
//! micropb-gen = "0.2.0"
//! ```
//!
//! Then, place your `.proto` file into the project's root directory:
//! ```proto
//! // example.proto
//! message Example {
//!     int32 field1 = 1;
//!     bool field2 = 2;
//!     double field3 = 3;
//! }
//! ```
//!
//! `micropb-gen` requires `protoc` to build `.proto` files, so [install
//! `protoc`](https://grpc.io/docs/protoc-installation) and add it to your PATH, then invoke the
//! code generator in `build.rs`:
//!
//! ```rust,no_run
//! let mut gen = micropb_gen::Generator::new();
//! // Compile example.proto into a Rust module
//! gen.compile_protos(&["example.proto"], std::env::var("OUT_DIR").unwrap() + "/example.rs").unwrap();
//! ```
//!
//! Finally, include the generated file in your code:
//! ```rust,ignore
//! // main.rs
//! use micropb::{MessageDecode, MessageEncode};
//!
//! mod example {
//!     #![allow(clippy::all)]
//!     #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
//!     // Let's assume that Example is the only message define in the .proto file that has been
//!     // converted into a Rust struct
//!     include!(concat!(env!("OUT_DIR"), "/example.rs"));
//! }
//!
//! let example = example::Example {
//!     field1: 12,
//!     field2: true,
//!     field3: 0.234,
//! };
//!
//! // Maximum size of the message type on the wire, scaled to the next power of 2 for heapless::Vec
//! const CAPACITY: usize = example::Example::MAX_SIZE.unwrap().next_power_of_two();
//! // For the example message above we can use a smaller capacity
//! // const CAPACITY: usize = 32;
//!
//! // Use heapless::Vec as the output stream
//! let mut data = heapless::Vec::<u8, CAPACITY>::new();
//!
//! // Compute the size of the `Example` on the wire
//! let _size = example.compute_size();
//! // Encode the `Example` to the data stream
//! example.encode_to_writer(&mut data).expect("Vec over capacity");
//!
//! // Decode a new instance of `Example` into a new struct
//! let mut new = example::Example::default();
//! new.decode_from_bytes(&mut decoder, data.as_slice()).expect("decoding failed");
//! assert_eq!(example, new);
//! ```
//!
//! # Messages
//!
//! Protobuf messages are translated directly into Rust structs, and each message field translates into a Rust field.
//!
//! Given the following Protobuf definition:
//! ```proto
//! syntax = "proto3";
//!
//! package example;
//!
//! message Example {
//!     int32 f_int32 = 1;
//!     int64 f_int64 = 2;
//!     uint32 f_uint32 = 3;
//!     uint64 f_uint64 = 4;
//!     sint32 f_sint32 = 5;
//!     sint64 f_sint64 = 6;
//!     bool f_bool = 7;
//!     fixed32 f_fixed32 = 8;
//!     fixed64 f_fixed64 = 9;
//!     sfixed32 f_sfixed32 = 10;
//!     sfixed64 f_sfixed64 = 11;
//!     float f_float = 12;
//!     double f_double = 13;
//! }
//! ```
//!
//! `micropb-gen` will generate the following Rust structs and APIs:
//! ```rust,ignore
//! pub mod example_ {
//!     #[derive(Debug, Clone)]
//!     pub struct Example {
//!         pub f_int32: i32,
//!         pub f_int64: i64,
//!         pub f_uint32: u32,
//!         pub f_uint64: u64,
//!         pub f_sint32: i32,
//!         pub f_sint64: i64,
//!         pub f_bool: bool,
//!         pub f_fixed32: u32,
//!         pub f_fixed64: u64,
//!         pub f_sfixed32: u32,
//!         pub f_sfixed64: u64,
//!         pub f_float: f32,
//!         pub f_double: f64,
//!     }
//!
//!     impl Example {
//!         /// Return reference to f_int32
//!         pub fn f_int32(&self) -> &i32;
//!         /// Return mutable reference to f_int32
//!         pub fn mut_f_int32(&mut self) -> &mut i32;
//!         /// Set value of f_int32
//!         pub fn set_f_int32(&mut self, val: i32) -> &mut Self;
//!         /// Builder method that sets f_int32. Useful for initializing the message.
//!         pub fn init_f_int32(mut self, val: i32) -> Self;
//!
//!         // Same APIs for the other singular fields
//!     }
//!
//!     impl Default for Example { /* ... */ }
//!
//!     impl PartialEq for Example { /* ... */ }
//!
//!     impl micropb::MessageEncode for Example { /* ... */ }
//!
//!     impl micropb::MessageDecode for Example { /* ... */ }
//! }
//! ```
//!
//! The generated [`MessageDecode`](micropb::MessageEncode) and
//! [`MessageEncode`](micropb::MessageDecode) implementations provide APIs for decoding, encoding,
//! and computing the size of `Example`.
//!
//! ## Optional Fields
//!
//! While the obvious choice for representing optional fields is [`Option`], this is not actually
//! ideal in embedded systems because `Option<T>` actually takes up twice as much space as `T` for
//! many types, such as `u32` and `i32`. Instead, **`micropb` tracks the presence of all optional
//! fields of a message in a separate bitfield called a _hazzer_**, which is usually small enough to
//! fit into the padding. Field presence can either be queried directly from the hazzer or from
//! message APIs that return `Option`.
//!
//! For example, given the following Protobuf message:
//! ```proto
//! message Example {
//!     optional int32 f_int32 = 1;
//!     optional int64 f_int64 = 2;
//!     optional bool f_bool = 3;
//! }
//! ```
//!
//! `micropb-gen` generates the following Rust struct and APIs:
//! ```rust,ignore
//! pub struct Example {
//!     pub f_int32: i32,
//!     pub f_int64: i64,
//!     pub f_bool: bool,
//!
//!     pub _has: Example_::_Hazzer,
//! }
//!
//! impl Example {
//!     /// Return reference to f_int32 as an Option
//!     pub fn f_int32(&self) -> Option<&i32>;
//!     /// Return mutable reference to f_int32 as an Option
//!     pub fn mut_f_int32(&mut self) -> Option<&mut i32>;
//!     /// Set value and presence of f_int32
//!     pub fn set_f_int32(&mut self, val: i32) -> &mut Self;
//!     /// Clear presence of f_int32
//!     pub fn clear_f_int32(&mut self) -> &mut Self;
//!     /// Take f_int32 and return it
//!     pub fn take_f_int32(&mut self) -> Option<i32>;
//!     /// Builder method that sets f_int32. Useful for initializing the message.
//!     pub fn init_f_int32(mut self, val: i32) -> Self;
//!
//!     // Same APIs for other optional fields
//! }
//!
//! pub mod Example_ {
//!     /// Tracks whether the optional fields are present
//!     #[derive(Debug, Default, Clone, PartialEq)]
//!     pub struct _Hazzer([u8; 1]);
//!
//!     impl _Hazzer {
//!         /// Create an empty Hazzer with all fields cleared
//!         pub const fn _new() -> Self;
//!         /// Query presence of f_int32
//!         pub const fn f_int32(&self) -> bool;
//!         /// Set presence of f_int32
//!         pub const fn set_f_int32(&mut self) -> &mut Self;
//!         /// Clear presence of f_int32
//!         pub const fn clear_f_int32(&mut self) -> &mut Self;
//!         /// Builder method that toggles on the presence of f_int32. Useful for initializing the Hazzer.
//!         pub const fn init_f_int32(mut self) -> Self;
//!
//!         // Same APIs for other optional fields
//!     }
//! }
//!
//! // trait impls, decode/encode logic, etc
//! ```
//!
//! ### Note on Initialization
//!
//! **A field will be considered empty (and ignored by the encoder) if its bit in the hazzer is not
//! set, _even if the field itself has been written_.** The following is an easy way to initialize a
//! message with all optional fields set:
//! ```rust,ignore
//! Example::default().init_f_int32(4).init_f_int64(-5).init_f_bool(true)
//! ```
//!
//! Alternatively, we can initialize the message using the constructor:
//! ```rust,ignore
//! Example {
//!     f_int32: 4,
//!     f_int64: -5,
//!     f_bool: true,
//!     // initialize the hazzer with all fields set to true
//!     // without initializing the hazzer, all fields in Example will be considered unset
//!     _has: Example_::_Hazzer::default()
//!             .init_f_int32()
//!             .init_f_int64()
//!             .init_f_bool()
//! }
//! ```
//!
//! ### Fallback to [`Option`]
//!
//! By default, optional fields are represented by bitfields, as shown above. If an optional field
//! is configured to be boxed via [`Config::boxed`], it will instead be represented as an `Option`,
//! because `Option<Box<T>>` doesn't take up extra space compared to `Box<T>`. To override these default
//! behaviours, see [`Config::optional_repr`].
//!
//! ### Required fields
//!
//! The generator treats required fields exactly the same way it treats optional fields.
//!
//! ## Oneof Fields
//!
//! Protobuf oneofs are translated into Rust enums. The enum type is defined in an internal
//! module under the message, and its type name is the same as the name of the oneof field.
//!
//! For example, given this Protobuf definition:
//! ```proto
//! message Example {
//!     oneof number {
//!         int32 int = 1;
//!         float decimal = 2;
//!     }
//! }
//! ```
//!
//! `micropb-gen` generates the following definition:
//! ```rust,no_run
//! #[derive(Debug, Clone, PartialEq)]
//! pub struct Example {
//!     pub number: Option<Example_::Number>,
//! }
//!
//! pub mod Example_ {
//!     #[derive(Debug, Clone, PartialEq)]
//!     pub enum Number {
//!         Int(i32),
//!         Decimal(f32),
//!     }
//! }
//! ```
//!
//! ## Repeated, `map`, `string`, and `bytes` Fields
//!
//! Repeated, `map`, `string`, and `bytes` fields need to be represented as Rust "container" types,
//! since they contain multiple elements or bytes. Normally standard types like `String` and `Vec`
//! are used, but they aren't available in no-alloc environments. Instead, we need stack-allocated
//! containers with fixed capacity. Since there is no defacto standard for such containers in Rust,
//! **users are expected to configure the code generator with their own container types** (see
//! [`Config`] for more details).
//!
//! For example, given the following Protobuf definition:
//! ```proto
//! message Containers {
//!     string f_string = 1;
//!     bytes f_bytes = 2;
//!     repeated int32 f_repeated = 3;
//!     map<int32, int64> f_map = 4;
//! }
//! ```
//!
//! and the following configuration in `build.rs`:
//! ```rust,no_run
//! let mut gen = micropb_gen::Generator::new();
//! // Configure our own container types
//! gen.configure(".",
//!     micropb_gen::Config::new()
//!         .string_type("crate::MyString<$N>")
//!         .bytes_type("crate::MyVec<u8, $N>")
//!         .vec_type("crate::MyVec<$T, $N>")
//!         .map_type("crate::MyMap<$K, $V, $N>")
//! );
//!
//! // We can also use container types from `heapless`, which have fixed capacity
//! gen.use_container_heapless();
//!
//! // Same shorthand exists for containers from `arrayvec` or `alloc`
//! // gen.use_container_arrayvec();
//! // gen.use_container_alloc();
//!
//!
//! // Since we're using fixed containers, we need to specify the max capacity of each field.
//! // For simplicity, configure capacity of all repeated/map fields to 4 and string/bytes to 8.
//! gen.configure(".", micropb_gen::Config::new().max_len(4).max_bytes(8));
//! ```
//!
//! The following Rust struct will be generated:
//! ```rust,no_run
//! # use micropb::heapless as heapless;
//! pub struct Containers {
//!     f_string: heapless::String<8>,
//!     f_bytes: heapless::Vec<u8, 8>,
//!     f_repeated: heapless::Vec<i32, 4>,
//!     f_map: heapless::FnvIndexMap<i32, i64, 4>,
//! }
//! ```
//!
//! For **decoding**, container types should implement [`PbVec`](micropb::PbVec) (repeated fields),
//! [`PbString`](micropb::PbString), [`PbBytes`](micropb::PbBytes), or [`PbMap`](micropb::PbMap)
//! For convenience, [`micropb`] comes with built-in implementations of the container traits for
//! types from [`heapless`](https://docs.rs/heapless/latest/heapless),
//! [`arrayvec`](https://docs.rs/arrayvec/latest/arrayvec), and
//! [`alloc`](https://doc.rust-lang.org/alloc), as well as implementations on `[u8; N]` arrays and
//! [`FixedLenString`](micropb::FixedLenString).
//!
//! For **encoding**, container types need to dereference into `&[T]` (repeated fields), `&str`, or
//! `&[u8]`. Maps just need to iterate through key-value pairs.
//!
//! ## Message Lifetime
//!
//! A message struct may have up to one lifetime parameter. `micropb-gen` automatically generates
//! the lifetime parameter for each message by checking if there's a lifetime in any of the fields.
//!
//! For example, given the Protobuf file from the previous section and the following `build.rs`
//! config:
//! ```rust,no_run
//! # use micropb_gen::{Generator, Config, config::CustomField};
//! # let mut gen = Generator::new();
//! // Use `Cow` as container type with lifetime of 'a
//! gen.configure(".",
//!     Config::new()
//!         .string_type("alloc::borrow::Cow<'a, str>")
//!         .bytes_type("alloc::borrow::Cow<'a, [u8]>")
//!         .vec_type("alloc::borrow::Cow<'a, [$T]>")
//! );
//! // Use a custom type for the `f_map` field, also with lifetime of 'a
//! gen.configure(".Containers.f_map",
//!     Config::new().custom_field(CustomField::from_type("MyField<'a>"))
//! );
//! ```
//!
//! `micropb-gen` generates the following struct:
//! ```rust,no_run
//! # extern crate alloc;
//! # struct MyField<'a>(&'a u8);
//! pub struct Containers<'a> {
//!     f_string: alloc::borrow::Cow<'a, str>,
//!     f_bytes: alloc::borrow::Cow<'a, [u8]>,
//!     f_repeated: alloc::borrow::Cow<'a, [i32]>,
//!     f_map: MyField<'a>,
//! }
//! ```
//!
//! ### Note
//! `micropb-gen` cannot detect if a message field contains a lifetime or not, so any field that's
//! a message with a lifetime must be configured with [`field_lifetime`](Config::field_lifetime).
//!
//! # Enums
//!
//! Protobuf enums are translated into "open" enums in Rust, rather than normal Rust enums. This is
//! because proto3 requires enums to store unrecognized values, which is only possible with open
//! enums.
//!
//! For example, given this Protobuf enum:
//! ```proto
//! enum Language {
//!     RUST = 0,
//!     C = 1,
//!     CPP = 2,
//! }
//! ```
//!
//! `micropb-gen` generates the following Rust definition:
//! ```rust,ignore
//! #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
//! #[repr(transparent)]
//! pub struct Language(pub i32);
//!
//! impl Language {
//!     // Default value
//!     pub const Rust: Self = Self(0);
//!     pub const C: Self = Self(1);
//!     pub const Cpp: Self = Self(2);
//! }
//!
//! impl From<i32> for Language { /* .. */ }
//! ```
//!
//! # Packages and Modules
//!
//! `micropb-gen` translates Protobuf package names into Rust modules by appending an underscore.
//!
//! For example, given the following Protobuf file:
//! ```proto
//! package foo.bar;
//!
//! // Protobuf contents
//! ```
//!
//! The generated Rust file will look like:
//! ```rust,ignore
//! pub mod foo_ {
//!     pub mod bar_ {
//!         // Generated code lives here
//!     }
//! }
//! ```
//!
//! If a Protobuf file does not have a package specifier, the generated code will instead live in
//! the root module
//!
//! Message names are also translated into Rust modules by appending an underscore. For example,
//! code generated from oneofs and nested messages within the `Name` message will live in the
//! `Name_` module.
//!
//! # Configuring the Generator
//!
//! One of `micropb-gen`'s main features is its granular configuration system, which allows users
//! to control how code is generated at the level of the module, message, or even individual
//! fields. See [`Generator::configure`] and [`Config`] for more info on the configuration system.
//!
//! ## Notable Configurations
//!
//! - **Integer size**: Controls the width of the integer types used to represent [integer
//! fields](Config::int_size). This can also be done for [enums](Config::enum_int_size).
//!
//! - **Attributes**: Apply custom attributes to [fields](Config::field_attributes) and
//! [messages](Config::type_attributes).
//!
//! - **Custom fields**: Substitute your own type into the generated code, allowing complete
//! control over the encode and decode behaviour. Can be applied to [normal
//! fields](Config::custom_field) or [unknown fields](Config::unknown_handler).
//!
//! - **Max container size**: Specify the max capacity of [`string`/`bytes`
//! fields](Config::max_bytes) as well as [repeated fields](Config::max_len), which is necessary
//! when using fixed-capacity containers like `ArrayVec`.

pub mod config;
mod generator;
mod pathtree;
mod utils;

// This module was generated from example/file-descriptor-proto
mod descriptor {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, dead_code, unused_imports)]
    include!("descriptor.rs");

    pub use google_::protobuf_::*;
}

use std::{
    env,
    ffi::OsStr,
    fmt, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

pub use config::Config;
pub use generator::Generator;
use micropb::{MessageDecode, PbDecoder};
use pathtree::PathTree;

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

type WarningCb = fn(fmt::Arguments);

fn warn_cargo_build(args: fmt::Arguments) {
    println!("cargo::warning={args}");
}

#[allow(clippy::new_without_default)]
impl Generator {
    /// Create new generator with default settings
    ///
    /// By default, the generator assumes it's running inside a Cargo build script, so all warnings
    /// will be emitted as compiler warnings. If the generator is not running inside a build
    /// script, use [`with_warning_callback`](Self::with_warning_callback).
    pub fn new() -> Self {
        Self::with_warning_callback(warn_cargo_build)
    }

    /// Create a generator with a custom callback for emitting warnings
    pub fn with_warning_callback(warning_cb: WarningCb) -> Self {
        let config_tree = PathTree::new(Box::new(Config::new()));
        Self {
            syntax: Default::default(),
            pkg_path: Default::default(),
            pkg: Default::default(),
            type_path: Default::default(),

            warning_cb,

            encode_decode: Default::default(),
            retain_enum_prefix: Default::default(),
            format: true,
            fdset_path: Default::default(),
            protoc_args: Default::default(),

            config_tree,
            extern_paths: Default::default(),
        }
    }

    /// Apply code generator configurations to Protobuf types and fields. See
    /// [`Config`](crate::Config) for possible configuration options.
    ///
    /// The `proto_path` argument is a fully-qualified Protobuf path that points to a package,
    /// type, or field in the compiled `.proto` files. The configurations are applied to the
    /// element specified by `proto_path`, as well as its children.
    ///
    /// # Example
    /// ```
    /// # use micropb_gen::{Generator, Config, config::IntSize};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // Configure field attributes on a specific field of a message type
    /// gen.configure(".pkg.Message.int_field", Config::new().field_attributes("#[serde(skip)]"));
    ///
    /// // Configure field attributes on all fields of a message type
    /// gen.configure(".pkg.Message", Config::new().field_attributes("#[serde(skip)]"));
    ///
    /// // Configure field attributes on all fields in a package
    /// gen.configure(".pkg", Config::new().field_attributes("#[serde(skip)]"));
    ///
    /// // Configure field attributes on all fields
    /// gen.configure(".", Config::new().field_attributes("#[serde(skip)]"));
    ///
    /// // Configure types attributes on a specific message type
    /// gen.configure(".pkg.Message", Config::new().type_attributes("#[derive(Serialize)]"));
    ///
    /// // Configure boxing behaviour on an oneof in a message type
    /// gen.configure(".pkg.Message.my_oneof", Config::new().boxed(true));
    ///
    /// // Configure the int size on a variant of an oneof
    /// gen.configure(".pkg.Message.my_oneof_variant", Config::new().int_size(IntSize::S8));
    ///
    /// // Configure the int size of an enum
    /// // Note that enum variants cannot be configured
    /// gen.configure(".pkg.Enum", Config::new().enum_int_size(IntSize::S8));
    /// ```
    ///
    /// # Special paths
    /// `configure` also supports special path suffixes for configuring fields in the generated
    /// code that don't have a corresponding Protobuf path.
    /// ```no_run
    /// # use micropb_gen::{Generator, Config, config::IntSize};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // Configure the int size of the elements in a repeated field via ".elem"
    /// gen.configure(".pkg.Message.repeated_field.elem", Config::new().int_size(IntSize::S8));
    ///
    /// // Configure the int size of the keys in a map field via ".key"
    /// gen.configure(".pkg.Message.map_field.key", Config::new().int_size(IntSize::S8));
    /// // Configure the int size of the values in a map field via ".value"
    /// gen.configure(".pkg.Message.map_field.value", Config::new().int_size(IntSize::S16));
    ///
    /// // Configure the field attributes of hazzer field and the type attributes of
    /// // the hazzer struct in the message via "._has"
    /// gen.configure(".pkg.Message._has",
    ///     Config::new().field_attributes("#[serde(skip)]").type_attributes("#[derive(Serialize)]"));
    ///
    /// // Configure the field attributes for the unknown handler field of the message via "._unknown"
    /// gen.configure(".pkg.Message._unknown", Config::new().field_attributes("#[serde(skip)]"));
    ///
    /// ```
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

    /// Apply one set of configurations to all provided Protobuf paths.
    ///
    /// See [`configure`](Self::configure) for how configurations are applied.
    pub fn configure_many(&mut self, proto_paths: &[&str], config: Config) -> &mut Self {
        for path in proto_paths {
            self.configure(path, config.clone());
        }
        self
    }

    /// Configure the generator to generate `heapless` containers for Protobuf `string`, `bytes`,
    /// repeated, and `map` fields.
    ///
    /// If using this option, `micropb` should have the `container-heapless` feature enabled.
    ///
    /// Specifically, `heapless::String<N>` is generated for `string` fields, `heapless::Vec<u8, N>`
    /// for `bytes` fields, `heapless::Vec<T, N>` for repeated fields, and
    /// `heapless::FnvIndexMap<K, V, N>` for `map` fields. This uses [`configure`](Self::configure)
    /// under the hood, so configurations set by this call can all be overriden.
    ///
    /// # Note
    /// Since `heapless` containers are fixed size, [`max_len`](Config::max_len) or
    /// [`max_bytes`](Config::max_bytes) must be set for all fields that generate these containers.
    pub fn use_container_heapless(&mut self) -> &mut Self {
        self.configure(
            ".",
            Config::new()
                .vec_type("::micropb::heapless::Vec<$T, $N>")
                .string_type("::micropb::heapless::String<$N>")
                .bytes_type("::micropb::heapless::Vec<u8, $N>")
                .map_type("::micropb::heapless::FnvIndexMap<$K, $V, $N>"),
        );
        self
    }

    /// Configure the generator to generate `arrayvec` containers for Protobuf `string`, `bytes`,
    /// and repeated fields.
    ///
    /// If using this option, `micropb` should have the `container-arrayvec` feature enabled.
    ///
    /// Specifically, `arrayvec::ArrayString<N>` is generated for `string` fields,
    /// `arrayvec::ArrayVec<u8, N>` for `bytes` fields, and `arrayvec::ArrayVec<T, N>` for repeated
    /// fields. This uses [`configure`](Self::configure) under the hood, so configurations set by
    /// this call can all be overriden.
    ///
    /// # Note
    /// No container is configured for `map` fields, since `arrayvec` doesn't have a suitable map
    /// type. If the .proto files contain `map` fields, [`map_type`](Config::map_type) will need to
    /// be configured separately.
    ///
    /// Since `arrayvec` containers are fixed size, [`max_len`](Config::max_len) or
    /// [`max_bytes`](Config::max_bytes) must be set for all fields that generate these containers.
    pub fn use_container_arrayvec(&mut self) -> &mut Self {
        self.configure(
            ".",
            Config::new()
                .vec_type("::micropb::arrayvec::ArrayVec<$T, $N>")
                .bytes_type("::micropb::arrayvec::ArrayVec<u8, $N>")
                .string_type("::micropb::arrayvec::ArrayString<$N>"),
        );
        self
    }

    /// Configure the generator to generate `alloc` containers for Protobuf `string`, `bytes`,
    /// repeated, and `map` fields.
    ///
    /// If using this option, `micropb` should have the `alloc` feature enabled.
    ///
    /// Specifically, `alloc::string::String` is generated for `string` fields,
    /// `alloc::vec::Vec<u8>` is for `bytes` fields, `alloc::vec::Vec<T>` for repeated fields, and
    /// `alloc::collections::BTreeMap<K, V>` for `map` fields. This uses
    /// [`configure`](Self::configure) under the hood, so configurations set by this call can all
    /// be overriden by future configurations.
    pub fn use_container_alloc(&mut self) -> &mut Self {
        self.configure(
            ".",
            Config::new()
                .vec_type("::alloc::vec::Vec<$T>")
                .bytes_type("::alloc::vec::Vec::<u8>")
                .string_type("::alloc::string::String")
                .map_type("::alloc::collections::BTreeMap<$K, $V>"),
        );
        self
    }

    /// Configure the generator to generate `std` containers for Protobuf `string`, `bytes`,
    /// repeated, and `map` fields.
    ///
    /// If using this option, `micropb` should have the `std` feature enabled.
    ///
    /// Specifically, `std::string::String` is generated for `string` fields, `std::vec::Vec<u8>`
    /// for `bytes` fields, `std::vec::Vec<T>` for repeated fields, and
    /// `std::collections::HashMap<K, V>` for `map` fields. This uses
    /// [`configure`](Self::configure) under the hood, so configurations set by this call can all
    /// be overriden by future configurations.
    pub fn use_container_std(&mut self) -> &mut Self {
        self.configure(
            ".",
            Config::new()
                .vec_type("::std::vec::Vec<$T>")
                .bytes_type("::std::vec::Vec::<u8>")
                .string_type("::std::string::String")
                .map_type("::std::collections::HashMap<$K, $V>"),
        );
        self
    }

    /// Compile `.proto` files into a single Rust file.
    ///
    /// # Example
    /// ```no_run
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

        let output = cmd.output().map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                io::Error::new(e.kind(), "`protoc` was not found. Check your PATH.")
            }
            _ => e,
        })?;
        if !output.status.success() {
            return Err(io::Error::other(format!(
                "protoc failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
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
        let mut decoder = PbDecoder::new(bytes.as_slice());
        let mut fdset = descriptor::FileDescriptorSet::default();
        fdset
            .decode(&mut decoder, bytes.len())
            .expect("file descriptor set decode failed");
        let code = self.generate_fdset(&fdset)?;

        self.warn_unused_configs();

        #[cfg(feature = "format")]
        let output = if self.format {
            dbg!(code.to_string());
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

    /// Declare an externally-provided Protobuf type.
    ///
    /// When compiling a `.proto` file that imports types from another `.proto` file, `micropb`
    /// won't compile the imported file if it's not included in the
    /// [`compile_protos`](Self::compile_protos) invocation. This is because the imported file may
    /// have already been compiled in another crate. In order to recognize externally-imported
    /// types, use `extern_type_path` to map the full Protobuf path of the imported type to the
    /// full path of the corresponding Rust type.
    ///
    /// # Example
    ///
    /// For example, let's say we have `app.proto`:
    /// ```proto
    /// // app.proto
    ///
    /// syntax = "proto3";
    /// package app;
    ///
    /// message App {
    ///     time.Timestamp timestamp = 1;
    ///     time.TZ timezone = 2;
    /// }
    /// ```
    ///
    /// `app.proto` imports from `time.proto`, which has already been compiled into the
    /// `time` crate:
    /// ```proto
    /// // time.proto
    ///
    /// syntax = "proto3";
    /// package time;
    ///
    /// message Timestamp {
    ///     uint32 ts = 1;
    /// }
    ///
    /// enum TZ {
    ///     TZ_UTC = 0;
    ///     TZ_PST = 1;
    /// }
    /// ```
    ///
    /// For our application, we're only interested in compiling `app.proto`, since `time.proto` has
    /// already been compiled by another crate. As such, we need to substitute Protobuf types
    /// imported from `time.proto` with Rust definitions from the `time` crate.
    /// ```no_run
    /// // build.rs of app
    ///
    /// let mut gen = micropb_gen::Generator::new();
    /// // Substitute Timestamp message
    /// gen.extern_type_path(".time.Timestamp", "time::Timestamp");
    /// // Substitute TZ enum
    /// gen.extern_type_path(".time.TZ", "time::Tz");
    /// // Compile only app.proto, not time.proto
    /// gen.compile_protos(&["app.proto"], std::env::var("OUT_DIR").unwrap() + "/output.rs").unwrap();
    /// ```
    ///
    /// # Note
    /// It's technically possible to substitute in Rust types that aren't generated by `micropb-gen`.
    /// However, the generated code expects substituted messages to implement `MessageDecode` and
    /// `MessageEncode`, and substituted enums to have the "open-enum" structure.
    pub fn extern_type_path<P1: AsRef<str>, P2: AsRef<str>>(
        &mut self,
        proto_path: P1,
        rust_path: P2,
    ) -> &mut Self {
        assert!(
            proto_path.as_ref().starts_with('.'),
            "Fully-qualified Proto path must start with '.'"
        );
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
