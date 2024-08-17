[![docs.rs](https://img.shields.io/docsrs/micropb-gen)](https://docs.rs/micropb-gen/latest/micropb-gen)
[![Crates.io Version](https://img.shields.io/crates/v/micropb-gen)](https://crates.io/crates/micropb-gen)

# micropb-gen

`micropb-gen` compiles `.proto` files into Rust. It is intended to be used inside `build.rs` for build-time code generation.

The entry point of this crate is the `Generator` type. Configuration of code generator behaviour is handled by the `Config` type.

**Note:** `micropb-gen` requires [`protoc`](https://grpc.io/docs/protoc-installation/) to be installed on the PATH to run the code generator.
