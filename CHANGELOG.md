# Changelog

## Unreleased

### Added

- Add `encode_cache` option to enable caching of message field lengths during encoding
- Add `compile_protos_with_config_files` to compile proto files with corresponding config files
- Add new unified error type `micropb_gen::Error`
- Add `micropb` support for heapless 0.9

### Changed

- Top-level APIs now return `micropb_gen::Error` instead of `io::Error`
- Split `container-heapless` into `container-heapless-0-8` and `container-heapless-0-9`
- Change `container-arrayvec` to `container-arrayvec-0-7`
- `use_container_heapless` now sets `map_type` to `heapless::index_map::FnvIndexMap`, which is the correct path in v0.9

### Removed

- Remove `micropb` re-exports of `heapless` and `arrayvec`

## 0.5.1

### Fixed

- Don't remove message module name suffixes when `suffixed_package_names` is turned off

## 0.5.0

### Removed

- Remove `field_lifetime` config
- Remove `recursive_field` config

### Changed

- `MAX_SIZE` changed from `Option<usize>` to `Result<usize, &str>` for reporting why the max size wasn't generated
- Lifetime params are now generated for parent messages if their child messages have lifetimes
- Applying `no_debug_impl`, `no_clone_impl`, and `no_partial_eq_impl` on a message will also apply to all its ancestors
- Recursively nested messages are automatically detected and prevented by boxing the field
- `MAX_SIZE` on recursive messages are set to `Err` to prevent cyclical references
- Increase `micropb-gen` to Rust 2024
- Bump MSRV to 1.85

## Added

- `Copy` derives are now generated for messages that consist purely of copy-able types

## 0.4.1

### Changed

- Comments in proto files are now used to generate doc comments in the Rust file (can be turned off)
- More doc comments in the generated Rust file
- Put generated message module after message declaration and impls

## 0.4.0

### Added

- Add `enum_unsigned` configuration
- Add support for TOML config files
- Add option to turn off suffixing for package names
- Add option to generate messages with single oneof as enums
- Add `None` to `OptionalRepr`, allowing non-optional representation of optional fields

## 0.3.0

### Added

- Add `no_accessors` configuration to reduce generated file size
- Add new setting to disable generating `MAX_SIZE` calculations

### Changed

- Use derives for `PartialEq` and `Default` when possible
- Replace `never` crate with `core::convert::Infallible`

## 0.2.0

### Added

- Add `ignore_wrong_len` flag to decoder
- Add convenience methods for decoding and encoding
- Add `MAX_SIZE` associated constant to `MessageEncode` trait, which statically computes the max size of a message on the wire
- Add `PbBytes` trait and `bytes_type` configuration
- Add `field_lifetime` configuration to set lifetime of message fields
- Add container trait impls for `Cow`
- Add const constructor `_new` to hazzer structs and add const to all hazzer methods
- Add `Generator::configure_many` method for configuring multiple paths at once
- Add `recursive_field` configuration to box and handle max size for recursive fields

### Changed

- Bump MSRV to 1.83 for const `Option::unwrap`
- Remove `PbContainer` and change definitions of all container traits
- Use string substitution for type and const params when configuring container types
- Revamped documentation

### Fixed

- Reject Protobuf Editions .proto files instead of pretending they're proto3
- Fix lifetime detection for messages and oneofs

## 0.1.2

### Added

- Add accessor APIs to singular fields
- Add `init_*` APIs for fields
- Add `FixedLenArray` and `FixedLenString` for string and bytes fields with static length

### Changed

- Return `&mut Self` from `set_*` and `clear_*` APIs

### Fixed

- Improved error message when protoc is not found
- Correct PartialEq implementations are now generated for message types
