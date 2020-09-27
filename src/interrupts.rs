use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame) {
    println!("Found breakponit\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64) -> ! {
    panic!(
        "EXCEPTION: Double fault.\nerror code: {}\n{:#?}",
        error_code, stack_frame,
    )
}

#[test_case]
fn test_breakpoint() {
    x86_64::instructions::interrupts::int3();
}
