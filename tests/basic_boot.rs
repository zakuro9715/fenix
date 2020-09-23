#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fenix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fenix::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fenix::handle_panic(info)
}

#[test_case]
fn test_println() {
    println!("test");
}