#![no_std]
#![no_main]

use core::panic::PanicInfo;

static OHNO: &[u8] = b"OH NO";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Nothing to do for now but
    // TODO: Print that we panic'd
    print(OHNO);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

fn print(s: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in s.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print(HELLO);

    loop {}
}

