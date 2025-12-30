![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/YuhanLiin/micropb/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/micropb)](https://docs.rs/micropb/latest/micropb)
[![Crates.io Version](https://img.shields.io/crates/v/micropb)](https://crates.io/crates/micropb)

# Micropb

`micropb` is a [Rust](https://www.rust-lang.org/) implementation of the [Protobuf](https://protobuf.dev/) format, with a focus on embedded environments. `micropb` generates a Rust module from `.proto` files.

Unlike other Rust Protobuf libraries, `micropb` is aimed for constrained environments where no allocator is available. As such, `micropb` offers a different set of tradeoffs compared to other Protobuf libraries.

#### Advantages
- Supports no-std and **no-alloc** environments.
- Reduced memory usage for generated code, especially for optional fields.
- Allows both statically-allocated containers ([`heapless`](https://docs.rs/heapless/latest/heapless), [`arrayvec`](https://docs.rs/arrayvec/latest/arrayvec)) or dynamically-allocated containers from [`alloc`](https://doc.rust-lang.org/alloc).
- Code generator is highly configurable.
- Fields can have custom handlers with user-defined encoding and decoding behaviour.
- Supports different data sources for encoding and decoding, abstracted behind the `PbRead` and `PbWrite` traits.
- Supports caching of message field lengths during encoding, improving performance on deeply nested message structures.

#### Limitations
- Does not support Protobuf Editions, RPC, or extensions.
- No reflection capabilities.
- `string`, `bytes`, repeated, and `map` fields require some basic user configuration to get working.

## Overview

The `micropb` project consists of two crates:

- [`micropb`](https://crates.io/crates/micropb): Encoding and decoding routines for the Protobuf wire data. The generated module will assume it's been imported as a regular dependency.

- [`micropb-gen`](https://crates.io/crates/micropb-gen): Code generation tool that generates a Rust module from a set of `.proto` files. Include this as a build dependency.

For a concrete example of `micropb` on an embedded application, see [`arm-app`](https://github.com/YuhanLiin/micropb/tree/main/examples/arm-app).

## Documentation

Documentation is at [docs.rs](https://docs.rs/micropb-gen/latest/micropb_gen/).

## MSRV

The oldest version of Rust that `micropb` supports is **1.88.0**.

## License

`micropb` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](https://github.com/YuhanLiin/micropb/blob/main/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/YuhanLiin/micropb/blob/main/LICENSE-MIT) for details.
