mkdir -p build
cargo build -Z build-std=core,alloc --release
mkdir -p build/boot/grub
cp grub.cfg build/boot/grub/grub.cfg
cp target/target/release/qos-rust.elf build/boot/qos-rust.elf
grub-mkrescue build -o qos-rust.iso
cp build/boot/qos-rust.elf qos-rust.elf
rm -rf build