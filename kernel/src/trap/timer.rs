use riscv::register::time;

pub fn get_time() -> usize {
    time::read()
}

// pub fn get_time_ms() -> usize {
//     get_time() / (CLOCK_FREQ / MSEC_PER_TICK)
// }

pub const CLOCK_FREQ: usize = 12500000;
pub const TICKS_PER_SEC: usize = 100;
// pub const MSEC_PER_TICK: usize = 1000;

pub fn set_next_trigger() {
    let time = get_time();
    crate::sbi::set_timer(time + CLOCK_FREQ / TICKS_PER_SEC);
}