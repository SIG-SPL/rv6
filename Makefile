GDB=riscv64-unknown-elf-gdb
BIOS=./src/asm/rustsbi-qemu.bin
QEMU = qemu-system-riscv64
DEBUGTARGET=./target/riscv64gc-unknown-none-elf/debug/rv6

run: 
	@cargo run --release

debug: 
	@cargo build
	@echo "*** Now run '$(GDB)' in another window." 1>&2
	$(QEMU) \
		-machine virt \
		-nographic \
		-bios $(BIOS) \
		-kernel $(DEBUGTARGET) \
		-s -S

test:
	@cargo test

clean:
	@cargo clean
