# micropb-gen

`micropb-gen` compiles `.proto` files into Rust. It is intended to be used inside `build.rs` for build-time code generation.

The entry point of this crate is the `Generator` type. Configuration of code generator behaviour is handled by the `Config` type.

**Note:** `micropb-gen` requires [`protoc`](https://grpc.io/docs/protoc-installation/) to be installed on the PATH to run the code generator.
