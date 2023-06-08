GDB=riscv64-unknown-elf-gdb
BIOS=rustsbi-qemu.bin
QEMU=qemu-system-riscv64
DEBUGTARGET=./target/riscv64gc-unknown-none-elf/debug/kernel

run: 
	@cargo run --release --bin kernel

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
	@echo "         _____         _     _  __                    _"
	@echo "        |_   _|__  ___| |_  | |/ /___ _ __ _ __   ___| |"
	@echo "          | |/ _ \/ __| __| | ' // _ \ '__| '_ \ / _ \ |"
	@echo "          | |  __/\__ \ |_  | . \  __/ |  | | | |  __/ |"
	@echo "          |_|\___||___/\__| |_|\_\___|_|  |_| |_|\___|_|"
	@echo "        ================================================"
	@cargo test --package kernel

clean:
	@cargo clean
