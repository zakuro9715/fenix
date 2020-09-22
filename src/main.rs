#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::run_tests)]
#![reexport_test_harness_main = "test_main"]


extern crate rlibc;
mod vga_buffer;
mod qemu;
mod serial;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("FenixOS {}\n", env!("CARGO_PKG_VERSION"));

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("\x1b[1G\x1b[1m\x1b[31m[Failed]\x1b[m"); // bold red -> Same as [OK]. See Testable
    serial_println!();
    serial_println!("Error: {}", info);
    serial_println!();

    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    loop {}
}


trait Testable {
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
fn run_tests(tests: &[&dyn Testable]) {
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
