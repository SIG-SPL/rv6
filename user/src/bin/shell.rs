#![no_std]
#![no_main]

#[macro_use]
extern crate ulib;

#[no_mangle]
pub extern "C" fn main() -> i32 {
    println!("Hello, RV6!");
    0
}
