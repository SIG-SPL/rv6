cargo build
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./src/asm/rustsbi-qemu.bin \
    -kernel ./target/riscv64gc-unknown-none-elf/debug/rv6 \
    -s -S

