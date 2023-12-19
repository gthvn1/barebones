# Kikuyu Colobus

The kikuyu Colobus monkey has its name that starts with 'k' that remember me the 'k' of Kernel... kind of.
It will be the operating system of Monkey Islang that one day will be the meeting of two monkeys,
[Kikuyu](https://github.com/gthvn1/kikuyu) and [Zanzibar](https://github.com/gthvn1/zanzibar).

## Overview

- Based on https://wiki.osdev.org/Bare_Bones
- Build with zig 0.12.0-dev
- `zig build run`
  - it requires *qemu-system-i386* to run
  - currently running as 32-bits because we only have *multiboot*

## Screenshots

- The first message...

![screenshot:first_msg](https://github.com/gthvn1/barebones/blob/master/screenshots/first_msg.png)

## Steps

### Step 1: bootstrap from OSDdev
- [x] Copy files
- [x] Build: `make`
  - It requires to cross compile so we built binutils, gdb and gcc
  - See https://wiki.osdev.org/GCC_Cross-Compiler
- [x] Run in qemu: `make run`

### Step2: Remove C and use Zig
- [x] Add the build.zig
- [x] Put zig source into src/
- [x] Build: `zig build`
- [x] Run in qemu: `qemu-system-i386 -kernel zig-out/bin/kikuyu`

### Step3: Print Hello from Zig
- [x] create `VgaTextMode` to manage the VGA text buffer
- [x] add function to set the color
- [x] add function to clear screen
- [x] add function to print one character
- [x] add function to print string

### Step4: Panic
- [x] if something goes wrong and panic is called we need to print the message

### Step?: ...
- Setup GDT (we rely on the one sets by the bootloader that is not good)

## Debug

- Check the address of the `kernelMain` function
```
❯ nm -s zig-out/bin/kikuyu | grep main
00200230 T kernelMain
```
- Start qemu with the option for debugging and stopping the CPU when started
```
❯ qemu-system-i386 -kernel zig-out/bin/kikuyu -s -S
```
- And connect remotly using gdb
```
❯ gdb -ex 'target remote localhost:1234'
```
- Now you can add a break to `kernelMain`, in my case `break *0x200230` and debug...
