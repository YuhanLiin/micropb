# Micropb

`micropb` is a [Rust](https://www.rust-lang.org/) implementation of the [Protobuf](https://protobuf.dev/) format, with a focus on embedded environments.

Unlike other Protobuf libraries, `micropb` is aimed for constrained environments where no allocator is available. Additionally, it aims to be highly configurable, allowing the user to customize the generated code on a per-field granularity. As such, `micropb` offers a different set of tradeoffs compared to other Protobuf libraries.

#### Advantages
- Supports no-std and **no-alloc** environments
- Reduced memory usage
- Allows both statically-allocated containers ([`heapless`](https://docs.rs/heapless/latest/heapless), [`arrayvec`](https://docs.rs/arrayvec/latest/arrayvec)) or dynamically-allocated containers from [`alloc`](https://doc.rust-lang.org/alloc)
- Code generator is highly configurable
- Fields can have custom handlers with user-defined encoding and decoding behaviour
- Can enable either encoder or decoder alone
- Can disable 64-bit integer operations

#### Limitations
- Depends on `protoc`
- Protobuf groups are not supported
- Unknown fields and extensions can only be captured with a custom handler
- Reflection is not supported
- Does not perform cycle detection, so users need to break cyclic references themselves by boxing the field or using a custom handler
- `string`, `bytes`, repeated, and `map` fields require some basic user configuration, as [explained later](#repeated-map-string-and-bytes-fields)

## Overview

The `micropb` project consists of two crates:

- **`micropb-gen`**: Code generation tool that generates a Rust module from a set of `.proto` files. Include this as a build dependency.

- **`micropb`**: Encoding and decoding routines for the Protobuf wire data. The generated module will assume it's been imported as a regular dependency.

### Getting Started

Add `micropb` crates to your `Cargo.toml`:
```toml
[dependencies]
micropb = "0.1"

[build-dependencies]
# Allow types from `heapless` to be used for container fields
micropb-gen = { version = "0.1", features = ["container-heapless"] }
```

`micropb-gen` requires `protoc` to build `.proto` files, so [install `protoc`](https://grpc.io/docs/protoc-installation) and add it to your PATH, then invoke the code generator in `build.rs`:
```rust,ignore
fn main() {
    let mut gen = micropb_gen::Generator::new();
    // Compile example.proto into a Rust module
    gen.compile_protos(&["example.proto"], std::env::var("OUT_DIR").unwrap() + "example.rs").unwrap();
}
```

Finally, include the generated file in your code:
```rust,ignore
// main.rs

use micropb::{PbRead, PbDecoder, MessageDecode, MessageEncode};

mod example {
    #![allow(clippy::all)]
    #![allow(warnings)]
    // Let's assume that Example is the only message define in the .proto file that has been 
    // converted into a Rust struct
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

fn main() {
    let mut example = example::Example::default();

    let data: &[u8] = &[ /* Protobuf data bytes */ ];
    // Construct new decoder from byte slice
    let mut decoder = PbDecoder::new(data);

    // Decode a new instance of `Example` into an existing struct
    example.decode(&mut decoder, data.len()).expect("decoding failed");

    // Use heapless::Vec as the output stream and build an encoder around it
    let mut encoder = PbEncoder::new(micropb::heapless::Vec::<u8, 10>::new());

    // Compute the size of the `Example` on the wire
    let size = example.compute_size();
    // Encode the `Example` to the data stream
    example.encode(&mut encoder).expect("Vec over capacity");
}
```

## Generated Code

### Messages

Protobuf messages are translated directly into Rust structs, and each message field translates into a Rust field.

Given the following Protobuf definition:
```proto
syntax = "proto3";

message Example {
    int32 f_int32 = 1;
    int64 f_int64 = 2;
    uint32 f_uint32 = 3;
    uint64 f_uint64 = 4;
    sint32 f_sint32 = 5;
    sint64 f_sint64 = 6;
    bool f_bool = 7;
    fixed32 f_fixed32 = 8;
    fixed64 f_fixed64 = 9;
    sfixed32 f_sfixed32 = 10;
    sfixed64 f_sfixed64 = 11;
    float f_float = 12;
    double f_double = 13;
}
```

`micropb` will generate the following Rust structs and APIs:
```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub struct Example {
    pub f_int32: i32,
    pub f_int64: i64,
    pub f_uint32: u32,
    pub f_uint64: u64,
    pub f_sint32: i32,
    pub f_sint64: i64,
    pub f_bool: bool,
    pub f_fixed32: u32,
    pub f_fixed64: u64,
    pub f_sfixed32: u32,
    pub f_sfixed64: u64,
    pub f_float: f32,
    pub f_double: f64,
}

impl Default for Example {
    // ...
}

impl micropb::MessageDecode for Example {
    // ...
}

impl micropb::MessageEncode for Example {
    // ...
}
```

The generated `MessageDecode` and `MessageEncode` implementations provide APIs for decoding, encoding, and computing the size of `Example`.

### Repeated, `map`, `string`, and `bytes` Fields

Repeated, `map`, `string`, and `bytes` fields require Rust "container" types, since they can contain multiple elements or characters. Normally standard types like `String` and `Vec` are used, but they aren't available on platforms without an allocator. In that case, statically-allocated containers with fixed size are needed. Since there is no defacto standard for static containers in Rust, users are expected to configure the code generator with their own container types.

For example, given the following Protobuf definition:
```proto
message Containers {
    string f_string = 1;
    bytes f_bytes = 2;
    repeated int32 f_repeated = 3;
    map<int32, int64> f_map = 4;
}
```

and the following configuration in `build.rs`:
```rust,ignore
// Use container types from `heapless`, which are statically-allocated
gen.use_container_heapless();

// We can also use container types from `arrayvec` or `alloc`
/*
gen.use_container_arrayvec();
gen.use_container_alloc();
*/

// We can even use our own container types
/*
gen.configure(".",
    micropb_gen::Config::new()
        .string_type("crate::MyString")
        .vec_type("crate::MyVec")
        .map_type("crate::MyMap")
);
*/

// Since we're using static containers, we need to specify the max capacity of each field.
// For simplicity, configure capacity of all repeated/map fields to 4 and string/bytes to 8.
gen.configure(".", micropb_gen::Config::new().max_len(4).max_bytes(8));
```

`micropb` will generate the following Rust definition:
```rust,no_run
pub struct Containers {
    f_string: heapless::String<8>,
    f_bytes: heapless::Vec<u8, 8>,
    f_repeated: heapless::Vec<i32, 4>,
    f_map: heapless::FnvIndexMap<i32, i64, 4>,
}
```

A container type is expected to implement `PbVec`, `PbString`, or `PbMap` from `micropb::container`, depending on what type of field it's used for. For convenience, `micropb` comes with built-in implementations of the container traits for types from [`heapless`](https://docs.rs/heapless/latest/heapless), [`arrayvec`](https://docs.rs/arrayvec/latest/arrayvec), and [`alloc`](https://doc.rust-lang.org/alloc) (see [Feature Flags](#feature-flags) for details).

### Optional Fields

Given the following Protobuf message:
```proto
message Example {
    optional int32 f_int32 = 1;
    optional int64 f_int64 = 2;
    optional bool f_bool = 3;
}
```

`micropb` generates the following Rust APIs:
```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub struct Example {
    pub f_int32: i32,
    pub f_int64: i64,
    pub f_bool: bool,

    pub _has: mod_Example::_Hazzer,
}

impl Example {
    /// Return reference to f_int32 as an Option
    pub fn f_int32(&self) -> Option<&i32>;
    /// Return mutable reference to f_int32 as an Option
    pub fn mut_f_int32(&mut self) -> Option<&mut i32>;
    /// Set value and presence of f_int32
    pub fn set_f_int32(&mut self, val: i32);
    /// Clear presence of f_int32
    pub fn clear_f_int32(&mut self);

    // Same APIs for other optional fields
}

pub mod mod_Example {
    /// Tracks whether the optional fields are present
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct _Hazzer([u8; 1]);

    impl _Hazzer {
        /// Query presence of f_int32
        pub fn f_int32(&self) -> bool;
        /// Set presence of f_int32
        pub fn set_f_int32(&mut self);
        /// Clear presence of f_int32
        pub fn clear_f_int32(&mut self);
        /// Builder method that toggles on the presence of f_int32. Useful for initializing the Hazzer.
        pub fn init_f_int32(mut self) -> Self;

        // Same APIs for other optional fields
    }
}
```

One big difference between `micropb` and other Protobuf libraries is that **`micropb` does not generate `Option` for optional fields**. This is because `Option<T>` takes up extra space if `T` doesn't have an invalid representation or unused bits. This is true for numeric types like `u32`, and can lead to the size of the field being doubled. For this reason, `micropb` tracks the presence of all optional fields in a separate bitfield called a "hazzer". Hazzers are usually small enough to fit into the message struct's padding, in which case it does not increase the size at all. Field presence can either be queried directly from the hazzer or from message APIs that return `Option`.

By default, boxed optional fields use `Option` to track presence, while other optional fields use hazzers. This behaviour can be overriden by the user via configuration.

#### Required Fields
Due to the problematic semantics of Protobuf's required fields, `micropb` will treat required fields exactly the same way it treats optional fields.

### Enums

Protobuf enums are translated into "open" enums in Rust, rather than normal Rust enums. This is because proto3 requires enums to be able to store unrecognized values, which is only possible with open enums.

For example, given this Protobuf enum:
```proto
enum Language {
    RUST = 0,
    C = 1,
    CPP = 2,
}
```

`micropb` generates the following Rust definition:
```rust,no_run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Language(pub i32);

impl Language {
    pub const Rust: Self = Self(0);
    pub const C: Self = Self(1);
    pub const Cpp: Self = Self(2);
}

// Default impl that returns the default variant

// From<i32> impl
```

The "enum" type is actually a thin struct wrapping an integer. Known enum variants are implemented as constants. Enum values can be created and matched in a similar manner as normal Rust enums. If the enum value is unknown, then the underlying integer value can be accessed directly from the struct.

### Oneof Fields

Protobuf oneofs are translated into real Rust enums. The enum type is defined in an internal module under the message, and its type name is the same as the name of the oneof field.

For example, given this Protobuf definition:
```proto
message Example {
    oneof number {
        int32 int = 1;
        float decimal = 2;
    }
}
```

`micropb` generates the following definition:
```rust,no_run
#[derive(Debug, Clone, PartialEq)]
pub struct Example {
    pub number: Option<mod_Example::Number>,
}

pub mod mod_Example {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Number {
        Int(i32),
        Decimal(f32),
    }
}
```

### Packages

`micropb` translates Protobuf packages into Rust modules. For example, if a Protobuf file has `package foo.bar`, all Rust types generated from the file will be in the `foo::bar` module. Code generated for Protobuf files without package specifiers will go into the module root.

### Nested Types

Rust does not allow a module to share its name with a struct, so nested messages and enums are defined in the `mod_Name` module, where `Name` is the message name. Oneof and hazzer definitions are also defined in `mod_Name`.

## Decoder and Encoder

`micropb` does not force a specific representation for Protobuf data streams. Instead, data streams are represented via read and write traits that users can implement, similar to [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) and [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) from the standard library. In addition, `micropb` provides decoder and encoder types that work on top of these traits to translate between the Protobuf data stream and Rust types. The decoder and encoder types are the main interface for accessing Protobuf data.

### `PbDecoder` and `PbRead`

Input data streams are represented by the `PbRead` trait, which is implemented on byte slices by default. The `PbDecoder` type wraps around an input stream and reads Protobuf structures from it, including message types generated by `micropb-gen`.

```rust,ignore
use micropb::{PbRead, PbDecoder, MessageDecode};
                                                                                                                                      
let data = [0x08, 0x96, 0x01, /* additional bytes */];
// Create decoder out of a byte slice, which is our input data stream
let mut decoder = PbDecoder::new(data.as_slice());

// ProtoMessage was generated by micropb
let mut message = ProtoMessage::default();
// Decode an instance of `ProtoMessage` from the data stream
message.decode(&mut decoder, data.len())?;

// We can also read Protobuf values directly from the decoder
let i = decoder.decode_int32()?;
let f = decoder.decode_float()?;
```

### `PbEncoder` and `PbWrite`

Output data streams are represented by the `PbWrite` trait, which is implemented on vector types from `alloc`, `heapless`, and `arrayvec` by default, depending on what feature flags are enabled. The `PbEncoder` type wraps around an output stream and writes Protobuf structures to it, including message types generated by `micropb-gen`.

```rust,ignore
use micropb::{PbEncoder, PbWrite, MessageEncode};
use micropb::heapless::Vec;
                                                                                                 
// Use heapless::Vec as the output stream and build an encoder around it
let mut encoder = PbEncoder::new(Vec::<u8, 10>::new());

// ProtoMessage was generated by micropb
let mut message = ProtoMessage::default();
message.0 = 12;
// Encode a `ProtoMessage` to the data stream
message.encode(&mut encoder)?;

// We can also write Protobuf values directly to the encoder
encoder.encode_int32(-4)?;
encoder.encode_float(12.491)?;
```

## Configuring the Code Generator

One of `micropb`'s main features is its granular configuration system. With it, users can control how code is generated from individual Protobuf messages and fields of their choosing. For example, if we have a message named `Example` with a field named `f_int32`, we can generate `Box<i32>` instead of `i32` for its type by putting the following in our `build.rs`:

```rust,ignore
generator.configure(".Example.f_int32", micropb_gen::Config::new().boxed(true));
```

We reference the `f_int32` field by using its full Protobuf path of `.Example.f_int32`. This allows configuration of any field or type in the compiled `.proto` files. Possible configuration options include: changing the representation of optional fields, setting the container type of repeated fields, setting field/type attributes, and changing the size of integer types.

For more info on how to configure code generated from Protobuf types and fields, refer to `Generator::configure` and `Config` in `micropb-gen`.

### Custom Field

In addition to configuring how fields get generated, users can also replace the field's generated type with their own custom type. For example, we can generate a custom type for `f_int32` as follows:

```rust,ignore
gen.configure(".Example.f_int32", micropb_gen::Config::new().custom_field(CustomField::Type("MyIntField<'a>".to_owned())));
```

This generates the following:
```rust,ignore
// If the custom field has a lifetime, then the message struct will also have a lifetime
pub struct Example<'a> {
    f_int32: MyIntField<'a>,
    // Rest of the fields
}
```

For more information on custom fields, see `Config::custom_field` in `micropb-gen`.

## Feature Flags

- **`encode`**: Enable support for encoding and computing the size of messages. If disabled, the generator should be configured to not generate encoding logic via `Generator::encode_decode`. Enabled by default.
- **`decode`**: Enable support for decoding messages. If disabled, the generator should be configured to not generate decoding logic via `Generator::encode_decode`. Enabled by default.
- **`enable-64bit`**: Enable 64-bit integer operations. If disabled, then 64-bit fields such as `int64` or `sint64` should have `Config::int_size` set to 32 bits or less. Has no effect on `double` fields. Enabled by default.
- **`alloc`**: Implements container traits on `Vec`, `String`, and `BTreeMap` from [`alloc`](https://doc.rust-lang.org/alloc), allowing them to be used as container fields. Corresponds with `Generator::use_container_alloc` from `micropb-gen`. Also implements `PbWrite` on `Vec`.
- **`std`**: Enables standard library and the `alloc` feature.
- **`container-heapless`**: Implements container traits on `Vec`, `String`, and `IndexMap` from [`heapless`](https://docs.rs/heapless/latest/heapless), allowing them to be used as container fields. Corresponds with `Generator::use_container_heapless` from `micropb-gen`. Also implements `PbWrite` on `Vec`.
- **`container-arrayvec`**: Implements container traits on `ArrayVec` and `ArrayString` from [`arrayvec`](https://docs.rs/arrayvec/latest/arrayvec), allowing them to be used as container fields. Corresponds with `Generator::use_container_arrayvec` from `micropb-gen`. Also implements `PbWrite` on `ArrayVec`.
