#!/usr/bin/env bash

set -xe

FLAGS=
if [ "$DISK" == "true" ]; then
  FLAGS="${FLAGS} -drive file=disk.img,if=ide,media=disk"
fi

if [ "$GDB" == "true" ]; then
  FLAGS="${FLAGS} -S -s -no-reboot -no-shutdown"
fi

if [ "$NET" == "true" ]; then
  FLAGS="${FLAGS} -netdev tap,id=net0,ifname=tap0,script=no,downscript=no -device rtl8139,netdev=net0,mac=12:34:45:67:89:ab" # you need tap0 set up
fi

if [ "$KVM" == "true" ]; then
  FLAGS="${FLAGS} -accel kvm -cpu host"
fi

if [ "$SVGA" == "true" ]; then
  FLAGS="${FLAGS} -device vmware-svga"
fi

mkdir -p build/boot/grub
cp compiletime/grub.cfg build/boot/grub/grub.cfg
cp $1 build/boot/qos.elf
grub-mkrescue build -o qos.iso
qemu-system-x86_64 -cdrom qos.iso -boot d -d guest_errors -debugcon stdio -serial null -machine q35 -device isa-debug-exit,iobase=0x501,iosize=1 ${FLAGS}
exit $(($? >> 1)) # holy bash
