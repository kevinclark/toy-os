#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
use vga_buffer::Color;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("{}", _info);
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    panic!("foo");

    loop {}
}

