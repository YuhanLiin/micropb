# Changelog

## [Unreleased]

### Added

- Add `ignore_wrong_len` flag to decoder
- Add convenience methods for decoding and encoding
- Add `MAX_SIZE` associated constant to `MessageEncode` trait, which statically computes the max size of a message on the wire

### Changed

- Bump MSRV to 1.83 for more const functions

### Fixed

- Reject Protobuf Editions .proto files instead of pretending they're proto3

## [0.1.2]

### Added

- Add accessor APIs to singular fields
- Add `init_*` APIs for fields
- Add `FixedLenArray` and `FixedLenString` for string and bytes fields with static length

### Changed

- Return `&mut Self` from `set_*` and `clear_*` APIs

### Fixed

- Improved error message when protoc is not found
- Correct PartialEq implementations are now generated for message types
