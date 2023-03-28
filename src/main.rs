#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}\n{}\n{}", "fuck", "hahaha", 1);
    loop {}
}
