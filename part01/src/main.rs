#![no_std]
#![no_main]

extern crate panic_halt;

static TEXT: &[u8] = b"We're Fucked up!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buff = 0xb8000 as *mut u8;

    for (idk, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *vga_buff.offset(idk as isize * 2) = byte;
            *vga_buff.offset(idk as isize * 2 + 1) = 0x0b;
        }
    }

    loop {}
}
