#!/usr/bin/env bash

mkdir -p build/boot/grub
cp compiletime/grub.cfg build/boot/grub/grub.cfg
cp $1 build/boot/qos.elf
grub-mkrescue build -o qos.iso
qemu-system-x86_64 -cdrom qos.iso -boot d -d guest_errors -debugcon stdio -serial null -vga std -machine q35 -no-reboot --enable-kvm -device isa-debug-exit,iobase=0x501,iosize=1
exit $(($? >> 1)) # holy bash
