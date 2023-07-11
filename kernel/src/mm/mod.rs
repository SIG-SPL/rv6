mod allocator;
mod vm;

pub fn init() {
    allocator::init();
    vm::init();
}