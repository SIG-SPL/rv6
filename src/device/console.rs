use core::fmt::{Arguments, Result, Write};

const UART: usize = 0x1000_0000;
const UART_THR: *mut u8 = UART as *mut u8;
const UART_LSR: *mut u8 = (UART + 0x05) as *mut u8;
const UART_LSR_EMPTY_MASK: u8 = 0x40;

struct DummyOut;

pub fn putc(ch: u8) -> u8 {
    while unsafe { *UART_LSR & UART_LSR_EMPTY_MASK } == 0 {}
    unsafe { *UART_THR = ch };
    ch
}

impl Write for DummyOut {
    fn write_str(&mut self, s: &str) -> Result {
        for &ch in s.as_bytes() {
            putc(ch);
        }
        Ok(())
    }
}

pub fn print(args: Arguments) {
    DummyOut.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::device::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
