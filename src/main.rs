#![no_std]
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// use core::panic::PanicInfo; 
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // todo!();  
    if cfg!(test) {
        blog_os::test_panic_handler(info); 
    } else {
        println!("{}", info); 
    }
    loop {} 
}

use blog_os::println; 
use blog_os::serial_println; 

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!"); 
    
    #[cfg(test)] 
    test_main(); 

    serial_println!("Hello World!"); 

    loop {} 
}
