.PHONY: run clean

AS=i686-elf-as
GCC=i686-elf-gcc

myos.bin: boot.o kernel.o linker.ld
	$(GCC) -T linker.ld -o myos.bin -ffreestanding -O2 -nostdlib boot.o kernel.o -lgcc

boot.o: boot.s
	$(AS) boot.s -o boot.o

kernel.o: kernel.c
	$(GCC) -c kernel.c -o kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra

run: myos.bin
	qemu-system-i386 -kernel myos.bin

clean:
	rm -f *.o myos.bin