#![no_std]
#![feature(panic_info_message)]
#![feature(linkage)]

use core::arch::asm;
use core::panic::PanicInfo;

pub const WRITE: usize = 1;
pub const OPEN: usize = 2;
pub const EXIT: usize = 93;

pub fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") id,
        );
    }
    ret
}

pub fn write(fd: usize, buffer: &[u8]) -> usize {
    syscall(WRITE, fd, buffer.as_ptr() as usize, buffer.len())
}

pub fn exit(code: i32) -> ! {
    syscall(EXIT, code as usize, 0, 0);
    panic!("unreachable after sys_exit!")
}

pub struct DummyWriter;

impl core::fmt::Write for DummyWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(1, s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = core::fmt::write(&mut $crate::DummyWriter, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        print!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        print!("Panicked: {}", info.message().unwrap());
    }
    exit(1)
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    exit(main())
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
