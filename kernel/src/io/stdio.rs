//! Interface for U-Mode standard I/O.
//! Calls to SYS_READ and SYS_WRITE are handled here.

use alloc::collections::VecDeque;

use crate::console::{CtrlChar, EscapeCode, InputMode};

pub struct Stdout;

impl core::fmt::Write for Stdout {
    #[cfg(feature = "graphics")]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut tb = super::graphics::TEXT_BUFFER.lock();
        for c in s.chars() {
            tb.putc(c);
        }
        Ok(())
    }

    #[cfg(not(feature = "graphics"))]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            crate::sbi::console_putchar(c as usize);
        }
        Ok(())
    }
}

/// TODO: Use lazy_static! after blocking & waking are implemented.
///   Otherwise, if we use SpinLock, deadlock will occur.
pub static mut STDIN: Stdin = Stdin::new();

/// TODO: Block a requesting thread if deque is empty,
///  and wake it up when new chars are available.
#[rustfmt::skip]
pub struct Stdin {
    /// Character deque.
    /// Chars in this deque are ready to be read.
    chars:  VecDeque<char>,
    state:  InputMode,
    /// Line buffer.
    /// Characters here are flushed to `chars` 
    /// when reaching a newline or calling `flush`.
    buffer: VecDeque<char>,
}

impl Stdin {
    const fn new() -> Self {
        Self {
            chars: VecDeque::new(),
            state: InputMode::default(),
            buffer: VecDeque::new(),
        }
    }

    // TODO: Support CtrlChars to manipulate the buffer
    pub fn push(&mut self, c: char) {
        match self.state {
            InputMode::Insert => self.pushc(c),
            // TODO: Replace mode
            InputMode::Replace => self.pushc(c),
            _ => self.recognize_escape(c),
        }
    }

    fn recognize_escape(&mut self, c: char) {
        match self.state {
            InputMode::EscapeState1 => {
                assert_eq!(c, EscapeCode::START);
                self.state = InputMode::EscapeState2;
            }
            InputMode::EscapeState2 => {
                match c {
                    EscapeCode::START => unreachable!(),
                    EscapeCode::VK_UP => {
                        // print!("up");
                    }
                    EscapeCode::VK_DOWN => {
                        // println!("down");
                    }
                    EscapeCode::VK_RIGHT => {
                        // println!("right");
                    }
                    EscapeCode::VK_LEFT => {
                        // println!("left");
                    }
                    _ => warn!("Unsupported escape code: \u{1b}{}", c),
                }
                self.state = InputMode::Insert;
            }
            _ => unreachable!(),
        }
    }

    fn pushc(&mut self, c: char) {
        match c {
            CtrlChar::CR | CtrlChar::LF => {
                if !self.buffer.is_empty() {
                    self.buffer.push_back('\n');
                    self.flush();
                }
            }
            CtrlChar::BS | CtrlChar::DEL => {
                if !self.buffer.is_empty() {
                    print!("{} {}", CtrlChar::BS, CtrlChar::BS);
                    self.buffer.pop_back();
                }
            }
            CtrlChar::HT => {
                for _ in 0..2 {
                    self.buffer.push_back(' ');
                }
            }
            CtrlChar::ESC => {
                self.state = InputMode::EscapeState1;
            }
            _ => self.buffer.push_back(c),
        }
    }

    pub fn flush(&mut self) {
        // Transfer chars from `buffer` to `chars`
        for c in self.buffer.drain(..) {
            self.chars.push_back(c);
        }
        // TODO: Wake up a blocked thread.
    }

    pub fn pop(&mut self) -> char {
        loop {
            let c = self.chars.pop_front();
            if let Some(ch) = c {
                return ch;
            } else {
                // TODO: Block the thread.
            }
            // Ideally, when a thread is woken up, the program counter
            //   goes here and tries to pop again.
            // The current implementation uses busy waiting.
        }
    }
}
