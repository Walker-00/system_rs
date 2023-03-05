#![no_std]
#![no_main]

mod vga_buff;

extern crate panic_halt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("As I told you, you've been Fucked!");
    println!("Memory bound in address {}", "0x256928");
    print!(".\n.\n.\n.\nDon't panic!, I'm just Kidding :\")");

    loop {}
}
