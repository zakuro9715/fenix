#![no_std]
#![no_main]

extern crate rlibc;
mod vga_buffer;


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = vga_buffer::Writer::new();
    writer.write_string("hello");
    loop {}
}
