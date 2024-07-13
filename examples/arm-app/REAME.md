Example of an embedded ARM application that uses `micropb`.

To build, run `cargo build --profile release-lto`. This will build for the `thumbv7em-none-eabihf` target.

To run the code, run `qemu-system-arm -cpu cortex-m4 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel <executable-file>` after building.

To check the size of the binary, run `cargo size --profile release-lto`. The current size is about 15kB.

To disassemble the binary, run `cargo objdump --profile release-lto -- -disassemble`.
