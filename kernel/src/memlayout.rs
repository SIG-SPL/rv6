pub const KERNBASE: usize = 0x80200000;
pub const PHY_SIZE: usize = 128 * 1024 * 1024;
pub const PHY_STOP: usize = KERNBASE + PHY_SIZE;

pub const KSTACKTOP: usize = PHY_STOP;
pub const PGSIZE: usize = 4 * 1024;
pub const STACKSIZE: usize = PGSIZE;

pub fn kstack(pid: usize) -> usize {
    KSTACKTOP - pid * STACKSIZE
}
