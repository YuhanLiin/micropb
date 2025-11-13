# Changelog

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
