#![no_std]
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// use core::panic::PanicInfo; 
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // todo!();  
    if cfg!(test) {
        serial_println!("[failed]\n");
        serial_println!("Error: {}\n", info);
        exit_qemu(QemuExitCode::Failed);
    } else {
        println!("{}", info); 
    }
    loop {} 
}

mod serial; 
mod vga_buffer; 

// use crate::vga_buffer::Writer; 

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!"); 
    
    #[cfg(test)] 
    test_main(); 

    loop {} 
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test(); 
    }
    exit_qemu(QemuExitCode::Success); 
}

#[test_case] 
fn trivial_assertion() {
    serial_print!("Trivial assert... "); 
    assert_eq!(0, 1); 
    serial_println! ("[OK]"); 
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
