#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fenix::panic_test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fenix::*;
use fenix::qemu::{exit_qemu, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_start_test_message(1);
    should_fail();
    print_test_result_label(TestResult::NG);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_test_result_label(TestResult::OK);
    print_complete_test_message();
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("\tshould_fail");
    assert!(false);
}
