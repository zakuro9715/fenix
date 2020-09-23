#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;
pub mod qemu;
pub mod serial;
pub mod vga_buffer;

mod tester;
pub use tester::*;

use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    handle_panic(info)
}

#[cfg(any(test, debug_assertions))]
pub fn handle_panic(info: &PanicInfo) -> ! {
    tester::fail_with_panic(info);
}

#[cfg(not(any(test, debug_assertions)))]
pub fn handle_panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}
