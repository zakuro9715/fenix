#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

const DEBUG_EXIT_IOBASE: u16 = 0xf4;

pub fn exit_qemu(code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(DEBUG_EXIT_IOBASE);
        port.write(code as u32);
    }
}
