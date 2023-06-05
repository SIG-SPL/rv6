# Rv6

A riscv operating system written in rust.

## Requirements
- [rust](https://www.rust-lang.org/tools/install)
- [qemu](https://www.qemu.org/download/)
- [riscv-gnu-toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain) (optional)

After installing rust, you need to install the `riscv64gc-unknown-none-elf` target:

```bash
rustup target add riscv64gc-unknown-none-elf
```


For MacOS users, you can install qemu and riscv-gnu-toolchain with homebrew.

```bash
brew install qemu   
brew tap riscv/riscv
brew install riscv-gnu-toolchain
```

## Run

```bash
cargo run --release
```

Debugging with gdb:

```bash
./debug.sh
riscv64-unknown-elf-gdb # in another terminal
```



## Features(Goal)
- process manage and context switch
- system call like xv6's 
- simple crash-free filesystem like xv6's
- simple bootstrap compiler running inside the os(so we have everything in principle)
- simple GUI
