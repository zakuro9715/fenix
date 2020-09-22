#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


extern crate rlibc;
pub mod vga_buffer;
pub mod qemu;
pub mod serial;


use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    handle_panic(info)
}

#[cfg(not(test))]
pub fn handle_panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn handle_panic(info: &PanicInfo) -> ! {
    serial_println!("\x1b[1G\x1b[1m\x1b[31m[Failed]\x1b[m"); // bold red -> Same as [OK]. See Testable
    serial_println!();
    serial_println!("Error: {}", info);
    serial_println!();

    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    loop {}
}


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        let name = core::any::type_name::<T>();
        serial_print!("\t  {}", name); // -> __________name
        if name.len() < 16 {
            serial_print!("\t");
        }
        self();
        serial_println!("\x1b[1G\x1b[1m\x1b[32m[OK]\x1b[m");  // bold green -> [ok]______name
    }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!();
    serial_println!("\x1b[1m\x1b[38;5;202mFenix test mode.\x1b[m"); // bold orange
    serial_println!("Running {} tests...", tests.len());
    serial_println!();

    for test in tests {
        test.run();
    }

    serial_println!();
    serial_println!("\x1b[1m\x1b[32mSuccess!\x1b[m"); // bold green

    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

#[test_case]
fn test_ok() {
    assert!(true);
}

#[cfg(feature="test_fail")]
#[test_case]
fn test_fail() {
    assert!(false, "test_fail feature enabeld");
}
