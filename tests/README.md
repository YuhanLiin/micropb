# Integration Tests

This folder contains the following test suites:

- `basic-proto`: Contains most of the test cases. If we need to add a new test, it should probably go in here. To add a new test case, add a new function to the build script to generate a new Rust output, then add a new test module that includes the new Rust output. New `.proto` files are added in the `proto/` directory.

- `serde-proto`: Test crate that generates Protobuf types with Serde support. For testing custom attributes on generated types.

- `encode-only`: Includes only encode logic, as well as disabling 64-bit ints. For testing encode-only and 32-bit only functionality.

- `decode-only`: Includes only decode logic, as well as disabling 64-bit ints. For testing decode-only and 32-bit only functionality.

- `proptest-proto`: Tests randomly-generated message structures and decoding of random bytes.
