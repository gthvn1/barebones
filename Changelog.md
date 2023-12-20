# Changelog

## 2023-12-20
- Implement serial driver
- Improve vga driver
- Update linker to use rust-llvm

## 2023-12-19

- Add vga module
    - print a welcome message on VGA screen
- Modify the build to target 32 bits kernel
    - We build 32 bits because qemu cannot load 64 bit ELF files by default. So
      switch to 32 bits is the easiest way for booting our kernel.
- include the boot.s from Zig version
    - We just copied it from our former Zig version and load it into `main.rs`.
    - It manages multiboot header and call kernel_main.
    - It uses AT&T syntax so add the options.
- Enabled memcpy, memset and memcmp
    - Some functions related to memory are not provided by default when building
    `compiler_builtins` because they are provided by the standard library and we
    avoid name collision. This is the case for `memset`, `memcpy` or `memcmp`.
    However we are now building our binary without the standard library so we can
    implement these functions (and maybe introduce bug) or enable their compilation.
    We choose the second option.
- First build
    - As we are building our own target we must also build the "core" and
    "compiler_builtins" crates. We don't have standard library but we need core and
    builtins.
        - This is done by adding `.cargo/config.toml` file
        - We also need to have rust source code:
        - `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`
