![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/YuhanLiin/micropb/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/micropb)](https://docs.rs/micropb/latest/micropb)
[![Crates.io Version](https://img.shields.io/crates/v/micropb)](https://crates.io/crates/micropb)

# Micropb

`micropb` is a [Rust](https://www.rust-lang.org/) implementation of the [Protobuf](https://protobuf.dev/) format, with a focus on embedded environments. `micropb` generates a Rust module from `.proto` files.

Unlike other Protobuf libraries, `micropb` is aimed for constrained environments where no allocator is available. Additionally, it aims to be highly configurable, allowing the user to customize the generated code on a per-field granularity. As such, `micropb` offers a different set of tradeoffs compared to other Protobuf libraries.

#### Advantages
- Supports no-std and **no-alloc** environments
- Reduced memory usage for generated code
- Allows both statically-allocated containers ([`heapless`](https://docs.rs/heapless/latest/heapless), [`arrayvec`](https://docs.rs/arrayvec/latest/arrayvec)) or dynamically-allocated containers from [`alloc`](https://doc.rust-lang.org/alloc)
- Code generator is highly configurable
- Fields can have custom handlers with user-defined encoding and decoding behaviour
- Supports different data sources for encoding and decoding, abstracted behind the `PbEncoder` and `PbDecoder` traits.
- Can enable either encoder or decoder alone

#### Limitations
- Some speed has been sacrificed for memory usage
- Does not support Protobuf editions for now
- Protobuf groups are not supported
- Unknown fields and extensions can only be captured with a custom handler
- Reflection is not supported
- Does not perform cycle detection, so users need to break cyclic references themselves by boxing the field or using a custom handler
- `string`, `bytes`, repeated, and `map` fields require some basic user configuration, as [explained later](#repeated-map-string-and-bytes-fields)

## Overview

The `micropb` project consists of two crates:

- [`micropb`](https://crates.io/crates/micropb): Encoding and decoding routines for the Protobuf wire data. The generated module will assume it's been imported as a regular dependency.

- [`micropb-gen`](https://crates.io/crates/micropb-gen): Code generation tool that generates a Rust module from a set of `.proto` files. Include this as a build dependency.

### Getting Started

Add `micropb` crates to your `Cargo.toml`:
```toml
[dependencies]
# Allow types from `heapless` to be used for container fields
micropb = { version = "0.1.0", features = ["container-heapless"] }

[build-dependencies]
micropb-gen = "0.1.0"
```

Then, place your `.proto` file into the project's root directory:
```proto
// example.proto
message Example {
    int32 field1 = 1;
    bool field2 = 2;
    double field3 = 3;
}
```

`micropb-gen` requires `protoc` to build `.proto` files, so [install `protoc`](https://grpc.io/docs/protoc-installation) and add it to your PATH, then invoke the code generator in `build.rs`:
```rust,ignore
fn main() {
    let mut gen = micropb_gen::Generator::new();
    // Compile example.proto into a Rust module
    gen.compile_protos(&["example.proto"], std::env::var("OUT_DIR").unwrap() + "/example.rs").unwrap();
}
```

Finally, include the generated file in your code:
```rust,ignore
// main.rs
use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod example {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    // Let's assume that Example is the only message define in the .proto file that has been 
    // converted into a Rust struct
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

fn main() {
    let example = example::Example {
        field1: 12,
        field2: true,
        field3: 0.234,
        ..Default::default()
    };

    // Maximum size of the message type on the wire, scaled to the next power of 2 due to heapless::Vec
    const CAPACITY: usize = example::Example::MAX_SIZE.unwrap().next_power_of_two();
    // For the example message above we can use a smaller capacity
    // const CAPACITY: usize = 32;

    // Use heapless::Vec as the output stream and build an encoder around it
    let mut encoder = PbEncoder::new(micropb::heapless::Vec::<u8, CAPACITY>::new());

    // Compute the size of the `Example` on the wire
    let size = example.compute_size();
    // Encode the `Example` to the data stream
    example.encode(&mut encoder).expect("Vec over capacity");

    let data = encoder.into_writer();
    // Construct new decoder from byte slice
    let mut decoder = PbDecoder::new(data.as_slice());

    // Decode a new instance of `Example` into a new struct
    let mut new = example::Example::default();
    new.decode(&mut decoder, data.len())
        .expect("decoding failed");
    assert_eq!(example, new);
}
```

For a concrete example of `micropb` on an embedded application, see [`arm-app`](https://github.com/YuhanLiin/micropb/tree/main/examples/arm-app).

## Generated Code

### Messages

Protobuf messages are translated directly into Rust structs, and each message field translates into a Rust field.

Given the following Protobuf definition:
```proto
syntax = "proto3";

package example;

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
pub mod example_ {
    #[derive(Debug, Clone)]
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

    impl PartialEq for Example {
        // ...
    }

    impl micropb::MessageDecode for Example {
        // ...
    }

    impl micropb::MessageEncode for Example {
        // ...
    }
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
        .string_type("crate::MyString<$N>")
        .bytes_type("crate::MyVec<u8, $N>")
        .vec_type("crate::MyVec<$T, $N>")
        .map_type("crate::MyMap<$K, $V, $N>")
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

A container type is expected to implement `PbVec`, `PbString`, `PbBytes`, or `PbMap` from `micropb::container`, depending on what type of field it's used for. For convenience, `micropb` comes with built-in implementations of the container traits for types from [`heapless`](https://docs.rs/heapless/latest/heapless), [`arrayvec`](https://docs.rs/arrayvec/latest/arrayvec), and [`alloc`](https://doc.rust-lang.org/alloc) (see [Feature Flags](#feature-flags) for details).

However, if only encoding logic is required, then the container traits are unnecessary. In that case, the only requirement for container types is that they dereference into `&[T]`, `&str`, and `&[u8]`, depending on what type of field it's for.

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
#[derive(Debug, Clone)]
pub struct Example {
    pub f_int32: i32,
    pub f_int64: i64,
    pub f_bool: bool,

    pub _has: Example_::_Hazzer,
}

impl Example {
    /// Return reference to f_int32 as an Option
    pub fn f_int32(&self) -> Option<&i32>;
    /// Return mutable reference to f_int32 as an Option
    pub fn mut_f_int32(&mut self) -> Option<&mut i32>;
    /// Set value and presence of f_int32
    pub fn set_f_int32(&mut self, val: i32) -> &mut Self;
    /// Clear presence of f_int32
    pub fn clear_f_int32(&mut self) -> &mut Self;
    /// Take f_int32 and return it
    pub fn take_f_int32(&mut self) -> Option<i32>;
    /// Builder method that sets f_int32. Useful for initializing the message.
    pub fn init_f_int32(mut self, val: i32) -> Self;

    // Same APIs for other optional fields
}

pub mod Example_ {
    /// Tracks whether the optional fields are present
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct _Hazzer([u8; 1]);

    impl _Hazzer {
        /// Query presence of f_int32
        pub fn f_int32(&self) -> bool;
        /// Set presence of f_int32
        pub fn set_f_int32(&mut self) -> &mut Self;
        /// Clear presence of f_int32
        pub fn clear_f_int32(&mut self) -> &mut Self;
        /// Builder method that toggles on the presence of f_int32. Useful for initializing the Hazzer.
        pub fn init_f_int32(mut self) -> Self;

        // Same APIs for other optional fields
    }
}
```

One big difference between `micropb` and other Protobuf libraries is that **`micropb` does not generate `Option` for optional fields**. This is because `Option<T>` takes up extra space for types like `i32` that don't have unused bits. Instead, `micropb` tracks the presence of all optional fields in a separate bitfield called a *hazzer*, which is usually small enough to fit into the message's padding. Field presence can either be queried directly from the hazzer or from message APIs that return `Option`.

Note that a field will be considered empty (and ignored by the encoder) if its bit in the hazzer is not set, even if the field itself has been written. The following is an easy way to initialize a message with all optional fields set:
```rust,ignore
Example::default().init_f_int32(4).init_f_int64(-5).init_f_bool(true)
```

Alternatively, we can initialize the message using the constructor by manually setting the bits in the hazzer:
```rust,ignore
Example {
    f_int32: 4,
    f_int64: -5,
    f_bool: true,
    // initialize the bitfield with all fields set to true
    // without this line, all fields in Example will be considered unset
    _has: Example_::_Hazzer::default()
            .init_f_int32()
            .init_f_int64()
            .init_f_bool()
}
```

#### Boxed optional fields
If an optional field is configured to be boxed, it will use `Option` instead of the hazzer to track presence, since `Option<Box<T>>` doesn't take up extra space.

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
    pub number: Option<Example_::Number>,
}

pub mod Example_ {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Number {
        Int(i32),
        Decimal(f32),
    }
}
```

### Packages

`micropb` translates Protobuf package names into Rust modules by appending an underscore. For example, if a Protobuf file has `package foo.bar;`, all Rust types generated from the file will be in the `foo_::bar_` module. Code generated for Protobuf files without package specifiers will go into the module root.

#### Nested Types

Message names are also translated into Rust modules by appending an underscore, so oneofs and nested messages/enums are defined in the `Name_` module, where `Name` is the message name.

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
let mut encoder = PbEncoder::new(Vec::<u8, 16>::new());

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

We reference the `f_int32` field by using its full Protobuf path of `.Example.f_int32`. This allows configuration of any field or type in the compiled `.proto` files. Possible configuration options include: changing optional fields from using hazzers to `Option`, setting the container type of repeated fields, adding field/type attributes, and changing the size of integer fields.

For more info on how to configure code generated from Protobuf types and fields, refer to [`Generator::configure`](https://docs.rs/micropb-gen/latest/micropb_gen/struct.Generator.html#method.configure) and [`Config`](https://docs.rs/micropb-gen/latest/micropb_gen/config/struct.Config.html) in `micropb-gen`.

### Custom Field

In addition to configuring how fields get generated, users can also replace the field's generated type with their own custom type. For example, we can generate a custom type for `f_int32` as follows:

```rust,ignore
gen.configure(
    ".Example.f_int32",
    micropb_gen::Config::new().custom_field(CustomField::Type("MyIntField<'a>".to_owned()))
);
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

- **encode**: Enable support for encoding and computing the size of messages. If disabled, the generator should be configured to not generate encoding logic via `Generator::encode_decode`. Enabled by default.
- **decode**: Enable support for decoding messages. If disabled, the generator should be configured to not generate decoding logic via `Generator::encode_decode`. Enabled by default.
- **enable-64bit**: Enable 64-bit integer operations. If disabled, then 64-bit fields such as `int64` or `sint64` should have `Config::int_size` set to 32 bits or less. Has no effect on `double` fields. Enabled by default.
- **alloc**: Implements container traits on `Vec`, `String`, and `BTreeMap` from [`alloc`](https://doc.rust-lang.org/alloc), allowing them to be used as container fields. Corresponds with `Generator::use_container_alloc` from `micropb-gen`. Also implements `PbWrite` on `Vec`.
- **std**: Enables standard library and the `alloc` feature.
- **container-heapless**: Implements container traits on `Vec`, `String`, and `IndexMap` from [`heapless`](https://docs.rs/heapless/latest/heapless), allowing them to be used as container fields. Corresponds with `Generator::use_container_heapless` from `micropb-gen`. Also implements `PbWrite` on `Vec`.
- **container-arrayvec**: Implements container traits on `ArrayVec` and `ArrayString` from [`arrayvec`](https://docs.rs/arrayvec/latest/arrayvec), allowing them to be used as container fields. Corresponds with `Generator::use_container_arrayvec` from `micropb-gen`. Also implements `PbWrite` on `ArrayVec`.

## MSRV

The oldest version of Rust that `micropb` supports is **1.83.0**.

## License

`micropb` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](https://github.com/YuhanLiin/micropb/blob/main/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/YuhanLiin/micropb/blob/main/LICENSE-MIT) for details.
