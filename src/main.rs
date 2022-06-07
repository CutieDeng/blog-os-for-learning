#![no_std]
#![no_main] 

// use core::panic::PanicInfo; 
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // todo!();  
    loop {} 
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{} 
}