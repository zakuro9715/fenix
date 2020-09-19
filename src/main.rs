#![no_std]
#![no_main]

extern crate rlibc;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("FenixOS {}\n", env!("CARGO_PKG_VERSION"));

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
