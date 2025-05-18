# Changelog

## [0.1.2]

### Added

- Add accessor APIs to singular fields
- Add `init_*` APIs for fields
- Add `FixedLenArray` and `FixedLenString` for string and bytes fields with static length

### Changed

- Return `&mut Self` from `set_*` and `clear_*` APIs

### Fixes

- Improved error message when protoc is not found
- Correct PartialEq implementations are now generated for message types
