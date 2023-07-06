use core::fmt::Write;

use crate::sbi::console_putchar;

#[rustfmt::skip]
#[allow(unused, non_snake_case)]
pub mod EscapeCode {
    pub const START   : char = '[';
    pub const VK_UP   : char = 'A';
    pub const VK_DOWN : char = 'B';
    pub const VK_RIGHT: char = 'C';
    pub const VK_LEFT : char = 'D';
}

#[rustfmt::skip]
#[allow(unused, non_snake_case)]
pub mod CtrlChar {
    pub const NUL: char = '\x00';
    pub const SOH: char = '\x01';
    pub const STX: char = '\x02';
    pub const ETX: char = '\x03';
    pub const EOT: char = '\x04';
    pub const ENQ: char = '\x05';
    pub const ACK: char = '\x06';
    pub const BEL: char = '\x07';
    pub const BS:  char = '\x08';
    pub const HT:  char = '\x09';
    pub const LF:  char = '\x0A';
    pub const VT:  char = '\x0B';
    pub const FF:  char = '\x0C';
    pub const CR:  char = '\x0D';
    pub const SO:  char = '\x0E';
    pub const SI:  char = '\x0F';
    pub const DLE: char = '\x10';
    pub const DC1: char = '\x11';
    pub const DC2: char = '\x12';
    pub const DC3: char = '\x13';
    pub const DC4: char = '\x14';
    pub const NAK: char = '\x15';
    pub const SYN: char = '\x16';
    pub const ETB: char = '\x17';
    pub const CAN: char = '\x18';
    pub const EM:  char = '\x19';
    pub const SUB: char = '\x1A';
    pub const ESC: char = '\x1B';
    pub const FS:  char = '\x1C';
    pub const GS:  char = '\x1D';
    pub const RS:  char = '\x1E';
    pub const US:  char = '\x1F';
    pub const DEL: char = '\x7F';  
}

#[allow(unused)]
pub enum InputMode {
    Insert,
    Replace,
    EscapeState1,
    EscapeState2,
}

impl InputMode {
    pub const fn default() -> Self {
        InputMode::Insert
    }
}

pub struct DummyOut;

impl Write for DummyOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::console::DummyOut.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[test_case]
fn test_println() {
    print!("test_println output");
}
