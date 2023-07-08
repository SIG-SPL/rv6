#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    kernel::load_address!(sp, boot_stack_top);
    kernel::call!(os_main);
    unreachable!()
}

#[no_mangle]
#[rustfmt::skip]
pub extern "C" fn os_main(hartid: usize, dtb_pa: usize) -> ! {
    #[cfg(test)]
    test_main();

    kernel::logging  ::init();
    kernel::allocator::init();
    kernel::io       ::init(dtb_pa);
    kernel::trap     ::init(hartid);
    kernel::fs       ::init();
    kernel::proc     ::init();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log::error!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        log::error!("Panicked: {}", info.message().unwrap());
    }
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
