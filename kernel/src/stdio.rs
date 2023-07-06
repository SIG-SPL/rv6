//! Interface for U-Mode standard I/O.
//! Calls to SYS_READ and SYS_WRITE are handled here.

extern crate alloc;

use alloc::collections::VecDeque;

pub struct Stdout;

impl core::fmt::Write for Stdout {
    #[cfg(feature = "graphics")]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut tb = crate::graphics::TEXT_BUFFER.lock();
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
pub struct Stdin {
    /// Char buffer.
    chars: VecDeque<char>,
}

impl Stdin {
    const fn new() -> Self {
        Self {
            chars: VecDeque::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.chars.push_back(c);
        // TODO: Wake up a blocked thread.
    }

    pub fn pop(&mut self) -> char {
        loop {
            let c = self.chars.pop_front();
            match c {
                Some(ch) => return ch,
                None => (), // TODO: Block the thread.
            }
            // Ideally, when a thread is woken up, the program counter
            //   goes here and tries to pop again.
            // The current implementation uses busy waiting.
        }
    }
}
