mkdir -p build
cargo build -Z build-std=core,alloc --release
mkdir -p build/boot/grub
cp grub.cfg build/boot/grub/grub.cfg
cp target/target/release/qos.elf build/boot/qos.elf
grub-mkrescue build -o qos.iso
cp build/boot/qos.elf qos.elf
rm -rf build