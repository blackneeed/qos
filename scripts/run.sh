mkdir -p build/boot/grub
cp compiletime/grub.cfg build/boot/grub/grub.cfg
cp $1 build/boot/qos.elf
grub-mkrescue build -o qos.iso
qemu-system-i386 -cdrom qos.iso -boot d -d guest_errors,cpu_reset,int -debugcon stdio -serial null -vga std -smp 2
