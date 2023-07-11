//! Tests for the test framework itself and the allocator.

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use config::fs::*;
use core::panic::PanicInfo;

extern crate alloc;

#[no_mangle]
pub extern "C" fn os_main(_hartid: usize, dtb_pa: usize) -> ! {
    kernel::allocator::init();
    kernel::io::init(dtb_pa);
    kernel::fs::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[test_case]
fn test_superblock() {
    use kernel::fs::*;
    let sb = Block::read_block(SUPER_BLOCK_NO);
    let sb = read_as::<SuperBlock>(&sb, 0);
    assert_eq!(sb.magic, FS_MAGIC);
    assert_eq!(sb.size, FSSIZE);
    assert_eq!(sb.nblocks, FSSIZE);
    assert_eq!(sb.ninodes, NINODES);
    assert_eq!(sb.nlog, LOGSIZE);
    assert_eq!(sb.logstart, 2);
    assert_eq!(sb.inodestart, 2 + LOGSIZE);
}

#[test_case]
fn test_root_inode() {
    use kernel::fs::*;
    let root = Inode::root();
    assert_eq!(root.inum, ROOTINO);
    assert_eq!(root.dinode.typ, FType::Dir);
    assert_eq!(root.dinode.major, 0);
    assert_eq!(root.dinode.minor, 0);
    assert_eq!(root.dinode.nlink, 1);
    assert_eq!(root.dinode.size, 0);
    assert_eq!(root.dinode.addrs[0], 0);
    let inode_bitmap = Block::read_block(INODE_BITMAP_START);
    assert_eq!(inode_bitmap.get(ROOTINO), 1);
    assert_eq!(inode_bitmap.get(0), 1);
}
