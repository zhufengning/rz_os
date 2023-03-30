#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use rz_os::{println, print, interrupts, gdt};
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[test_case]
fn double_fault() {
    stackoverflow();
}


//#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

use core::{panic::PanicInfo, ptr::read_volatile};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:#?}", info);
    loop {}
}

fn init() {
    gdt::init();
    interrupts::init_idt();
}

fn stackoverflow() {
    stackoverflow();
    let a:u64 = 0;
    unsafe {read_volatile(&a);}
}

#[test_case]
fn pagefalult() { 
    unsafe {
        *(0xdeadbeef as *mut i64) = 42;
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    init();
    #[cfg(test)]
    test_main();

    println!("{}\n{}\n{}", "fuck", "hahaha", 1);
    println!("bye~");

    x86_64::instructions::interrupts::int3();
    loop {}
}
