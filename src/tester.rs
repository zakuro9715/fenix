#[cfg(any(test, debug_assertions))]
mod internal {
    use crate::{qemu, serial_print, serial_println};
    use core::panic::PanicInfo;

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn fail_with_panic(info: &PanicInfo) -> ! {
        print_test_result_label(TestResult::NG);
        serial_println!();
        serial_println!("Error: {}", info);
        serial_println!();

        qemu::exit_qemu(qemu::QemuExitCode::Failed);
        loop {}
    }

    pub fn print_test_name<T>() {
        let name = core::any::type_name::<T>();
        serial_print!("      {}", name); // put 6 spaces for label like " [OK] "
        if name.len() < 16 {
            serial_print!("\t");
        };
    }

    pub enum TestResult {
        OK,
        NG,
    }

    pub fn print_test_result_label(status: TestResult) {
        use TestResult::{NG, OK};
        match status {
            OK => serial_println!("\x1b[1G\x1b[1m\x1b[32m [OK] \x1b[m"), // bold green
            NG => serial_println!("\x1b[1G\x1b[1m\x1b[31m [NG] \x1b[m"), // bold red
        }
    }

    pub trait Testable {
        fn run(&self) -> ();
    }

    impl<T> Testable for T
    where
        T: Fn(),
    {
        fn run(&self) {
            print_test_name::<T>();
            self();
            print_test_result_label(TestResult::OK)
        }
    }

    pub fn print_start_test_message(n: usize) {
        serial_println!();
        serial_println!("\x1b[1m\x1b[38;5;202mFenix test mode.\x1b[m"); // bold orange
        serial_println!("Running {} tests...", n);
        serial_println!();
    }

    pub fn print_complete_test_message() {
        serial_println!();
        serial_println!("\x1b[1m\x1b[32mSuccess!\x1b[m"); // bold green
    }

    pub fn test_runner(tests: &[&dyn Testable]) {
        print_start_test_message(tests.len());
        for test in tests {
            test.run();
        }
        print_complete_test_message();
        qemu::exit_qemu(qemu::QemuExitCode::Success);
    }

    pub fn panic_test_runner(tests: &[&dyn Testable]) {
        print_start_test_message(tests.len());
        for test in tests {
            test.run();
            print_test_result_label(TestResult::NG);
            qemu::exit_qemu(qemu::QemuExitCode::Failed);
        }
    }
}

#[cfg(any(test, debug_assertions))]
pub use internal::*;

#[cfg(not(any(test, debug_assertions)))]
pub fn test_runner(_: &[&dyn Fn()]) {
}

#[test_case]
fn test_ok() {
    assert!(true);
}

#[cfg(feature = "test_fail")]
#[test_case]
fn test_fail() {
    assert!(false, "test_fail feature enabeld");
}
