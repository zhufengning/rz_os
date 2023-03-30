use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt, println};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler_fn)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("Exception: Breakpoint\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler_fn(stack_frame: InterruptStackFrame, err_code: u64) -> ! {
    panic!("Exception: DoubleFault {}\n{:#?}", err_code, stack_frame);
}

pub fn init_idt() {
    IDT.load();
}
