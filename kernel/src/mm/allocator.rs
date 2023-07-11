//! TODO: use our own allocator and make a benchmark

use buddy_system_allocator::LockedHeap;
use config::layout::*;

use alloc::alloc::{alloc, Layout};

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[repr(align(4096), C)]
struct _PAGE {
    _data: [u8; PGSIZE],
}

const PAGE_LAYOUT: Layout = Layout::new::<_PAGE>();

pub fn init() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

pub fn alloc_page() -> usize {
    unsafe { alloc(PAGE_LAYOUT) as usize }
}
