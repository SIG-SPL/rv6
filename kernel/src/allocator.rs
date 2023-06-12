//! TODO: use our own allocator and make a benchmark

use buddy_system_allocator::LockedHeap;
// use crate::config::KERNEL_HEAP_SIZE;

const KERNEL_HEAP_SIZE: usize = 0x80_0000;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}
