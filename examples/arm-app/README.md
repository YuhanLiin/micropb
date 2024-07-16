Example of an embedded ARM application that uses `micropb`.

To build, run `cargo build --profile release-lto`. This will build for the `thumbv7em-none-eabihf` target.

To run the code, run `qemu-system-arm -cpu cortex-m4 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel <executable-file>` after building.

To check the size of the binary without including formatting/printing code, run `cargo size --profile release-lto --no-default-features 00 -- -A`. The current size of the `.text` section is 8.7 kB.

To disassemble the binary without formatting/printing code, run `cargo objdump --profile release-lto --no-default-features -- --disassemble`.
