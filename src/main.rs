#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(rz_os::test_runner)]

mod gdt;
use core::panic::PanicInfo;
use rz_os::{println, init};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    
    #[cfg(test)]
    test_main();

    println!("Welcome to this OS.");
    println!("bye~");
    x86_64::instructions::interrupts::int3();
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:#?}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use rz_os::{serial_println, exit_qemu, QemuExitCode};

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}