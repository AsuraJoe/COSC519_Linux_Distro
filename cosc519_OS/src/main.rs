#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cosc519_OS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cosc519_OS::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    cosc519_OS::init();
    // fn stack_overflow(){
    //     stack_overflow();
    // }

    // stack_overflow();
    // x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();

    println!("It did not crash~Yay");
    cosc519_OS::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    cosc519_OS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cosc519_OS::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}