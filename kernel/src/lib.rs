#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(core_intrinsics)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("asm/start.S"));
global_asm!(include_str!("asm/swtch.S"));

#[macro_use]
pub mod util;
#[macro_use]
pub mod console;
pub mod allocator;
mod context;
pub mod fs;
pub mod io;
mod loader;
pub mod logging;
pub mod proc;
pub mod sbi;
pub mod sched;
mod sync;
mod syscall;
pub mod trap;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

extern crate alloc;

pub use context::{Context, TrapFrame};

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("test {} ...\t", core::any::type_name::<T>());
        self();
        println!("\x1b[32mok\x1b[0m");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    crate::sbi::shutdown()
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn os_main() -> ! {
    test_main();
    crate::sbi::shutdown()
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("\x1b[31m[failed]\x1b[0m");
    println!("{}\n", info);
    crate::sbi::shutdown()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
