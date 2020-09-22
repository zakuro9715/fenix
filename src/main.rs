#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::run_tests)]
#![reexport_test_harness_main = "test_main"]


extern crate rlibc;
mod vga_buffer;
mod qemu;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("FenixOS {}\n", env!("CARGO_PKG_VERSION"));

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
fn run_tests(tests: &[&dyn Fn()]) {
    println!("Fenix test.\nRunning {} tests...", tests.len());
    for test in tests {
        test();
    }
    println!("Success!");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

#[test_case]
fn test_example() {
    assert_eq!(1, 1);
}
