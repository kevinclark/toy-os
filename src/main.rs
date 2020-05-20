#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
use vga_buffer::Color;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga_buffer::foreground_color(Color::Red);
    println!("{}", _info);

    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::with_colors(Color::Blue, Color::Green, || {
        print!("Hello world{}", "!");
    });

    println!();

    panic!("WOOO");

    loop {}
}

