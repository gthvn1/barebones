#!/bin/sh

# We have an issue to test with qemu because qemu returns the error we passed
# is left shifted by 1 bit and or'd with 1.  So even if we pass 0 to qemu,
# it returns 1. This wrapper script works around that issue and returns so
# we can use cargo test now !!!
qemu-system-i386 \
  -serial stdio \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -kernel $1

ret=$?

if [ $ret -eq 1 ]; then
    exit 0
else
    exit 1
fi
