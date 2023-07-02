GDB=riscv64-unknown-elf-gdb
QEMU=qemu-system-riscv64
DEBUGTARGET=./target/riscv64gc-unknown-none-elf/debug/kernel

symbol:
	@cargo objdump --bin kernel --quiet -- -d > kernel.asm 2>/dev/null
	@cargo nm --bin kernel --quiet > System.map 2>/dev/null

run: symbol
	@cargo run --bin kernel

release: symbol
	@cargo run --release --bin kernel

nographic: symbol
	@$(QEMU) \
		-serial mon:stdio \
		-nographic \
		-machine virt \
		-drive file=fs.img,format=raw,id=hd0 \
        -device virtio-blk-device,drive=hd0 \
		-kernel $(DEBUGTARGET)

debug: 
	@cargo build
	@echo "*** Now run '$(GDB)' in another window." 1>&2
	$(QEMU) \
		-serial mon:stdio \
		-nographic \
		-machine virt \
		-drive file=fs.img,format=raw,id=hd0 \
        -device virtio-blk-device,drive=hd0 \
		-kernel $(DEBUGTARGET) \
		-s -S

debug-graphic: 
	@cargo build
	@echo "*** Now run '$(GDB)' in another window." 1>&2
	$(QEMU) \
		-serial mon:stdio \
		-machine virt \
		-drive file=fs.img,format=raw,id=hd0 \
		-device virtio-blk-device,drive=hd0 \
		-device virtio-gpu-device \
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
	@rm -f kernel.asm System.map
