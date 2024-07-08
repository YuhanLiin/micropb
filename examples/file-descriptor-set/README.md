The build script of this example compiles Google's `descriptor.proto` into Rust. The compiled Rust module is used to process the file descriptor sets produced by `protoc` as part of the compilation pipeline in `micropb-gen`. Thus, this example acts to "bootstrap" `micropb-gen`.

To update `micropb-gen`, simply copy `descriptor.proto` into `<workspace-root>/micropb-gen/src/`.
