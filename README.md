# Barebones

- Based on https://wiki.osdev.org/Bare_Bones

# Steps

## Step 1: bootstrap from OSDdev
- [x] Copy files
- [x] Build: `make`
  - It requires to cross compile so we built binutils, gdb and gcc
  - See https://wiki.osdev.org/GCC_Cross-Compiler
- [x] Run in qemu: `make run`

## Step2: Remove C and use Zig
- [x] Add the build.zig
- [x] Put zig source into src/
- [x] Build: `zig build`
- [x] Run in qemu: `qemu-system-i386 -kernel zig-out/bin/myos.bin`

## Step3: Print Hello from Zig
- [ ] add functions to print string
