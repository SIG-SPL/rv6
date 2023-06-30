# Rv6

A riscv operating system written in rust.

## Requirements
- [rust](https://www.rust-lang.org/tools/install)
- [qemu](https://www.qemu.org/download/), especially `qemu-system-riscv64`.
- [riscv-gnu-toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain) (optional but recommended).

After installing rust, you need to switch to nightly and install the `riscv64gc-unknown-none-elf` target:

```bash
rustup override set nightly # switch to nightly in this directory
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
```

For MacOS users, you can install qemu and riscv-gnu-toolchain with homebrew.

```bash
brew install qemu   
brew tap riscv/riscv
brew install riscv-gnu-toolchain
```

## Run

```bash
make run
```

Debugging with gdb:

```bash
make debug
riscv64-unknown-elf-gdb # in another terminal
```

## Features(Goal)
- [ ] process manage and context switch
- [ ] system call like xv6's
- [ ] simple crash-free filesystem like xv6's
- [ ] **simple bootstrap compiler running inside the os(so we have everything in principle)**
- [ ] simple GUI


## References
- [xv6](https://github.com/mit-pdos/xv6-riscv) An **awesome** teaching os written in C.
- [Writing an OS in Rust](https://os.phil-opp.com/) A great blog about writing os in rust.
- [rCore](https://github.com/rcore-os) A rust os project with Chinese documents.
