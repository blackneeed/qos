wsl bash -lc ./build.sh
qemu-system-x86_64 -cdrom qos.iso -boot d -d guest_errors,cpu_reset,int -debugcon stdio -machine pc -vga std -m 4G -drive file=disk.img,if=ide,media=disk
