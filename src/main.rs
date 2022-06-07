#![no_std]
#![no_main] 

// use core::panic::PanicInfo; 
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // todo!();  
    println!("{}", info); 
    loop {} 
}


mod vga_buffer; 

// use crate::vga_buffer::Writer; 

#[no_mangle]
pub extern "C" fn _start() -> ! {

    // let mut t = Writer::new(); 

    // use core::fmt::Write; 
    // for _ in 0..10 {
    //     write!(t, "Hello, I'm {}! \n", "Cutie Deng").unwrap(); 
    // }

    // write!(vga_buffer::WRITER.lock(), "Hello World!\n").unwrap(); 
    println!("Hello World!"); 

    panic!("Some panic Message. "); 
    
    loop {} 
}