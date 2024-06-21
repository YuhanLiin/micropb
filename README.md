# Micropb

`micropb` is a Rust implementation of the Protobuf format, with a focus on embedded and `nostd` use-cases. `micropb` generates Rust code from Protobuf files.

Unlike other Protobuf libraries, `micropb` is aimed for constrained environments where the standard library isn't available. Additionally, it aims to be maximally configurable, allowing the user to customize the generated code on a per-field granularity. As such, `micropb` offers a different set of advantages and limitations than other similar libraries.

### Advantages
- Generated code does not rely on `std` or `alloc` by default
- Reduced memory usage
- Allows both statically-allocated containers (`heapless`, `arrayvec`) or dynamically-allocated containers from `alloc`
- Code generator is highly configurable
- Fields can have custom handlers with user-defined encoding and decoding behaviour
- Can enable either encoder or decoder alone
- Can disable 64-bit integer operations

### Limitation
- Some speed has been traded off for memory usage
- Protobuf groups are not supported
- Unknown fields and extensions can only be captured with a custom handler
- Reflection is not supported
- Does not perform cycle detection, so users need to break cyclic references themselves by boxing the field or using a custom handler
- `string`, `bytes`, repeated, and `map` fields require some basic configuration to work, as explained later

# Overview

`micropb` consists of two components:

- `micropb-gen`: Code generation tool that generates a Rust module from a set of Protobuf files. Include this as a build dependency.

- `micropb`: Encoding and decoding routines for the Protobuf format. The generated module will assume it's been imported as a regular dependency.

## Using `micropb`

Add `micropb` components to your `Cargo.toml`:
```toml
[dependencies]
micropb = "0.1"

[build-dependencies]
micropb-gen = "0.1"
```

Then invoke the code generator in `build.rs`:
```rust,no_run
fn main() {
    let mut gen = micropb_gen::Generator::new();
    gen.compile_protos(&["example.proto"], std::env::var("OUT_DIR").unwrap() + "example.rs").unwrap();
}
```

Finally, include the generated file in your code:
```rust,no_run
// lib.rs

mod example {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

// rest of your code
```

# Generated Code

## Messages

Protobuf messages are translated directly into Rust structs, and each message field translates into a Rust field.

Given the following Protobuf definition:
```proto
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
```rust,no_run
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
```

## Repeated, `map`, `string`, and `bytes` Fields

Repeated, `map`, `string`, and `bytes` fields require Rust "container" types, since they can contain multiple elements or characters. Normally standard types like `String` and `Vec` are used, but they aren't available on platforms without an allocator. In that case, statically-allocated containers with fixed size are needed. Since there is no defacto standard for static containers in Rust, `micropb` expects users to either opt into its built-in support for `heapless` and `arrayvec`, or provide their own container types.

For example, given the following Protobuf definition:
```proto
message Containers {
    string f_string = 1;
    bytes f_bytes = 2;
    repeated int32 f_repeated = 3;
    map<int32, float> f_map = 4;
}
```

and the following configuration in `build.rs`:
```rust,ignore
// Use container types from `heapless`, which are statically-allocated
gen.use_container_heapless();
// Since we're using static containers, we need to specify the max capacity of each field
// For simplicity, configure capacity of all repeated/map fields to 5 and string/bytes to 8
gen.configure(".", micropb_gen::Config::new().max_len(5).max_bytes(8));
```

`micropb` will generate the following Rust definition:
```
#[derive(Debug, Clone, PartialEq)]
pub struct Containers {
    f_string: heapless::String<8>,
    f_bytes: heapless::Vec<u8, 8>,
    f_repeated: heapless::Vec<i32, 5>,
    f_map: heapless::IndexMap<i32, f32, 5>,
}

// Default impl
```

## Optional Fields

Given the following Protobuf message:
```proto
message Example {
    optional int32 f_int32 = 1;
    optional int64 f_int64 = 2;
    optional bool f_bool = 3;
}
```

`micropb` generates the following Rust definition:
```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub struct Example {
    pub f_int32: i32,
    pub f_int64: i64,
    pub f_bool: bool,

    pub _has: mod_Example::_Hazzer,
}

impl Example {
    // Get optional reference to f_int32
    pub fn f_int32(&self) -> Option<&i32>;
    // Get optional mutable reference to f_int32
    pub fn mut_f_int32(&mut self) -> Option<&mut i32>;
    // Set f_int32 to a value
    pub fn set_f_int32(&mut self, val: i32);
    // Clear value of f_int32
    pub fn clear_f_int32(&mut self);

    // Similar APIs for other optional fields
}

pub mod mod_Example {
    // Tracks whether the optional fields are present
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct _Hazzer([u8; 1]);

    impl _Hazzer {
        // Turn on presence of f_int32
        pub fn f_int32(&self) -> bool;
        // Turn off presence of f_int32
        pub fn set_f_int32(&self, flag: bool);

        // Similar APIs for other optional fields
    }
}
```

One big difference between `micropb` and other Protobuf libraries is that, by default, `micropb` does not generate `Option`s for optional fields. This is because `Option<T>` takes up extra space if `T` doesn't have an invalid representation or unused bits. For numeric types like `u32`, it can even double the size of the field. Instead, `micropb` tracks the presence of all optional fields in separate bitfields called a "hazzer". Hazzers are usually small enough to fit into the message struct's padding, in which case it does not increase the size at all. Field presence can either be queried directly from the hazzer or from struct APIs that return `Option`.

By default, boxed optional fields use `Option` to track presence, while other optional fields use hazzers. This behaviour can be changed by the user.

### Required Fields
Due to the problematic semantics of Protobuf's required fields, `micropb` will treat required fields exactly the same way it treats optional fields.

## Enums

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

The "enum" type is actually a thin struct wrapping an integer. Known enum variants are invoked with expressions like `Language::Rust`. As such, enum values can be created and matched in a similar manner as normal Rust enums. If the enum value is unknown, then the underlying integer value can be accessed directly.

## Oneof Fields

Protobuf oneofs are translated into real Rust enums. The enum is defined in an internal module under the message, and its type name is the same as the name of the oneof field.

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

## Packages

`micropb` translates Protobuf packages into Rust modules. For example, if a Protobuf file has `package foo.bar`, all Rust types generated from the file will be in the `foo::bar` module. Code generated for Protobuf files without package specifiers will go into the module root.

## Nested Types

Rust does not allow a module to share its name with a struct, so nested messages and enums are defined in the `mod_Name` module, where `Name` is the message name. Oneof and hazzer definitions are also defined in `mod_Name`.
