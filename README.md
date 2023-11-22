# Barebones

- Based on https://wiki.osdev.org/Bare_Bones

# Steps

## Step 1: bootstrap from OSDdev
- [*] Copy files
- [*] Build: `make`
  - It requires to cross compile so we built binutils, gdb and gcc
  - See https://wiki.osdev.org/GCC_Cross-Compiler
- [*] Run in qemu: `make run`

## Step2: Remove C and use Zig
- [*] Add the build.zig
- [*] Put zig source into src/
- [*] Build: `zig build`
- [ ] Run in qemu
  - With Zig we built a 64 bits version. The multiboot 1 doesn't so
    we need to use multiboot2
- [ ] Write Hello world from Zig
- [ ] Cleanup old files