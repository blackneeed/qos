CFLAGS=-g -std=c11 -Wall -Wextra -ffreestanding -fno-stack-protector -fno-stack-check -fno-lto -m32 -mno-mmx -mno-sse -mno-sse2 -mno-red-zone -mno-avx -mno-80387 -I src
LDFLAGS=-nostdlib -m elf_i386 -static
ASMFLAGS=-g -f elf32
CSRC=$(shell find src -name *.c)
COBJ=$(patsubst src/%.c,obj/%.c.o,$(CSRC))
ASMSRC=$(shell find src -name *.asm)
ASMOBJ=$(patsubst src/%.asm,obj/%.asm.o,$(ASMSRC))

.PHONY: build
build: kernel inject_bootloader clean

.PHONY: dirs
dirs:
	mkdir -p build obj

obj/%.c.o: src/%.c
	mkdir -p $(shell dirname '$@')
	gcc $(CFLAGS) -c -o $@ $< 

obj/%.asm.o: src/%.asm
	mkdir -p $(shell dirname '$@')
	nasm $(ASMFLAGS) -o $@ $<

link_kernel: $(COBJ) $(ASMOBJ)
	ld $(LDFLAGS) -T linker.ld $(ASMOBJ) $(COBJ) -o build/QuickOS.elf

.PHONY: kernel
kernel: dirs $(COBJ) $(ASMOBJ) link_kernel

.PHONY: inject_bootloader
inject_bootloader: kernel
	mkdir -p build/iso
	mkdir -p build/iso/boot
	mkdir -p build/iso/boot/grub
	cp grub.cfg build/iso/boot/grub/grub.cfg
	cp build/QuickOS.elf build/iso/boot/QuickOS.elf
	cp build/QuickOS.elf QuickOS.elf 
	grub-mkrescue build/iso -o QuickOS.iso

.PHONY: run
run: kernel inject_bootloader
	qemu-system-x86_64 -cdrom QuickOS.iso -boot d -d guest_errors,cpu_reset,int

.PHONY: clean
clean: kernel inject_bootloader
	rm -rf build obj