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
  FLAGS="${FLAGS} -netdev user,id=net0 -device rtl8139,netdev=net0,mac=$(python3 scripts/random_mac.py)"
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
qemu-system-x86_64 -cdrom qos.iso -debugcon stdio -M q35 ${FLAGS}
exit $(($? >> 1))
