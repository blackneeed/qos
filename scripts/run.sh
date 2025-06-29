mkdir -p build/boot/grub
cp grub.cfg build/boot/grub/grub.cfg
cp $1 build/boot/qos.elf
grub-mkrescue build -o qos.iso
qemu-system-x86_64 -cdrom qos.iso -boot d -d guest_errors,cpu_reset,int -debugcon file:/dev/stdout -vga std
