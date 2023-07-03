//! config show the all SETTING that all project used, it includes:
//! * Interface of CPU and OS, i.e. SBI
//! * Interface of OS and user applications, such as system call number
//! * Layout of OS kernel
//! * Standard input/output/error settings, such as stdin, stdout, stderr

#![no_std]

pub mod sbi {
    // legacy extensions: ignore fid
    pub const SBI_SET_TIMER: usize = 0;
    pub const SBI_CONSOLE_PUTCHAR: usize = 1;
    pub const SBI_CONSOLE_GETCHAR: usize = 2;
    pub const SBI_CLEAR_IPI: usize = 3;
    pub const SBI_SEND_IPI: usize = 4;
    pub const SBI_REMOTE_FENCE_I: usize = 5;
    pub const SBI_REMOTE_SFENCE_VMA: usize = 6;
    pub const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;

    // system reset extension
    pub const SRST_EXTENSION: usize = 0x53525354;
    pub const SBI_SHUTDOWN: usize = 0;
}

/* Interface of operating system and applications */
#[rustfmt::skip]
pub mod syscall {
    /// syscall number
    pub const SYSCALL_EXIT: usize     = 93;
    pub const SYSCALL_WRITE: usize    = 64;
    pub const SYSCALL_READ: usize     = 63;
    pub const SYSCALL_OPEN: usize     = 56;
    pub const SYSCALL_CLOSE: usize    = 57;
    pub const SYSCALL_YIELD: usize    = 124;
    pub const SYSCALL_FORK: usize     = 220;
    pub const SYSCALL_EXEC: usize     = 221;
    pub const SYSCALL_WAITPID: usize  = 260;
    pub const SYSCALL_GETPID: usize   = 172;
    pub const SYSCALL_SLEEP: usize    = 101;
    pub const SYSCALL_SBARK: usize    = 400;
    pub const SYSCALL_GETTIME: usize  = 169;
    pub const SYSCALL_GETCWD: usize   = 17;
    /// syscall register index
    pub const SYSCALL_REG_NUM: usize = 17; // a7
    pub const SYSCALL_REG_ARG0: usize = 10; // a0
    pub const SYSCALL_REG_ARG1: usize = 11;
    pub const SYSCALL_REG_ARG2: usize = 12;
    pub const SYSCALL_REG_ARG3: usize = 13;
    pub const SYSCALL_REG_ARG4: usize = 14;
    pub const SYSCALL_REG_ARG5: usize = 15;
    pub const SYSCALL_REG_ARG6: usize = 16;
    pub const SYSCALL_REG_RET: usize  = 10;
}

pub mod layout {
    /*
     * ============= Physical Address Layout in QEMU =============
     * 
     *  0x7ffffffff +------------------+
     *              |       ...        |
     *  0x87ffffff  +------------------+ <- PHY_STOP
     *              |    FREE SPACE    |
     *              |------------------|
     *              |      KERNEL      |
     *  0x80200000  |------------------| <- KERNEL_BASE
     *              |    BOOTLOADER    |    (i.e., OpenSBI)
     *  0x80000000  |------------------| <- riscv_virt_board.ram
     *              |       ...        |
     *  0x100081ff  |------------------|
     *              |       MMIO       |
     *  0x10001000  |------------------| <- virtio-mmio
     *              |       ...        |
     *  0x10000007  |------------------|
     *              |       UART       |
     *  0x10000000  |------------------| <- serial
     *              |       ...        |
     *  0x0c5fffff  |------------------|
     *              |       PLIC       |
     *  0x0c000000  |------------------| <- riscv.sifive.plic
     *              |       ...        |
     *  0x0000ffff  |------------------|
     *              |     FIRMWARE     |
     *  0x00001000  +------------------+ <- riscv_virt_board.mrom
     * 
     */
    pub const PLIC_BASE: usize = 0xc000000;
    pub const PLIC_SENABLE_BASE: usize = PLIC_BASE + 0x2080;
    pub const PLIC_SPRIORITY_BASE: usize = PLIC_BASE + 0x201000;
    pub const UART0: usize = 10;
    pub const VIRTIO0: usize = 1;

    pub const KERNEL_BASE: usize = 0x80200000;
    pub const PHY_SIZE: usize = 128 * 1024 * 1024;
    pub const PHY_STOP: usize = KERNEL_BASE + PHY_SIZE;

    pub const KSTACKTOP: usize = PHY_STOP;
    /// Page size 4KB
    pub const PGSIZE: usize = 4 * 1024;
    /// Kernel stack size 4KB
    pub const STACKSIZE: usize = PGSIZE;
    /// Kernel Heap size 3MB
    pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

    pub fn plic_pri(intr_src: usize) -> usize {
        PLIC_BASE + intr_src * 4
    }

    pub fn plic_sen(hartid: usize) -> usize {
        PLIC_SENABLE_BASE + hartid * 0x100
    }

    pub fn plic_spri(hartid: usize) -> usize {
        PLIC_SPRIORITY_BASE + hartid * 0x2000
    }
}

/* Standard input/output/error settings */
pub mod std_io {
    pub const STDIN: usize = 0;
    pub const STDOUT: usize = 1;
    pub const STDERR: usize = 2;
}
