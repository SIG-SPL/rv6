DEBUGTARGET   = ./target/riscv64gc-unknown-none-elf/debug/kernel
RElEASETARGET = ./target/riscv64gc-unknown-none-elf/release/kernel

GDB = riscv64-unknown-elf-gdb

QEMU = qemu-system-riscv64
QEMUOPTS =  -serial mon:stdio -machine virt 
QEMUOPTS += -drive file=fs.img,format=raw,id=hd0 
QEMUOPTS += -device virtio-blk-device,drive=hd0
GPUOPTS  =  -device virtio-gpu-device

# Build in debug mode. Debug mode disables GPU by default.
build:
	@cd kernel && cargo build
	@cd kernel && cargo objdump --quiet -- -d > ../kernel.asm 2>/dev/null
	@cd kernel && cargo nm --quiet > ../System.map 2>/dev/null

run: build 
	@$(QEMU) $(QEMUOPTS) -nographic -kernel $(DEBUGTARGET)

release:
	@cd kernel && cargo build --release --features graphics
	@$(QEMU) $(QEMUOPTS) $(GPUOPTS) -kernel $(RElEASETARGET)

debug: build
	@echo "*** Now run '$(GDB)' in another window." 1>&2
	$(QEMU) $(QEMUOPTS) -nographic -kernel $(DEBUGTARGET) -s -S

test:
	@echo "         _____         _     _  __                    _"
	@echo "        |_   _|__  ___| |_  | |/ /___ _ __ _ __   ___| |"
	@echo "          | |/ _ \/ __| __| | ' // _ \ '__| '_ \ / _ \ |"
	@echo "          | |  __/\__ \ |_  | . \  __/ |  | | | |  __/ |"
	@echo "          |_|\___||___/\__| |_|\_\___|_|  |_| |_|\___|_|"
	@echo "        ================================================"
	@cd kernel && cargo test

fs:
	@cd mkfs && cargo run fs.img
	@mv mkfs/fs.img .
	@cp fs.img kernel/fs.img

clean:
	@rm -r target
	@rm -f kernel.asm System.map
