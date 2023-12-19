# Links

- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Bare Bones - booting simple kernel](https://wiki.osdev.org/Bare_Bones)
- [Writing an OS - Sphaerophoria YT](https://www.youtube.com/watch?v=gBykJMqDqH0&list=PL980gcR1LE3LBuWuSv2CL28HsfnpC4Qf7)

# Notes

## Memory
- Some functions related to memory are not provided by default when building `compiler_builtins` because
they are provided by the standard library and we avoid name collision. This is the case for `memset`,
`memcpy` or `memcmp`. However we are now building our binary without the standard library so we can
implement these functions (and maybe introduce bug) or enable their compilation. We choose the second
option.

## Build
- As we are building our own target we must also build the "core" and "compiler_builtins" crates. We don't have standard library but we need core and builtins.
  - This is done by adding `.cargo/config.toml` file
  - We also need to have rust source code:
    - `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`
- To build:
```
kikuyu/rust/kikuyu master*
❯ cargo +nightly build --target x86_64-kikuyu.json
    Updating crates.io index
  Downloaded getopts v0.2.21
  Downloaded rustc-demangle v0.1.23
  Downloaded unicode-width v0.1.10
  Downloaded adler v1.0.2
  Downloaded addr2line v0.21.0
  Downloaded miniz_oxide v0.7.1
  Downloaded allocator-api2 v0.2.15
  Downloaded memchr v2.5.0
  Downloaded hashbrown v0.14.3
  Downloaded gimli v0.28.0
  Downloaded compiler_builtins v0.1.103
  Downloaded object v0.32.1
  Downloaded 12 crates (1.2 MB) in 1.15s
   Compiling compiler_builtins v0.1.103
   Compiling core v0.0.0 (/home/gthvn1/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling rustc-std-workspace-core v1.99.0 (/home/gthvn1/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling kikuyu v0.1.0 (/home/gthvn1/devel/monkey_islang/kikuyu/rust/kikuyu)
    Finished dev [unoptimized + debuginfo] target(s) in 7.78s
```
