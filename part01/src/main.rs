#![no_std]
#![no_main]

use core::{panic::PanicInfo, *};
use vga_buff::ColorCode;

mod vga_buff;

static TEXT: &[u8] = b"BLABLA";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let bla = 0xb8000 as *mut u8;

    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *bla.offset(i as isize * 2) = byte;
            *bla.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    let mut writer = vga_buff::Writer {
        clmn_pst: 0,
        color_code: ColorCode::new(vga_buff::Color::Red, vga_buff::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut vga_buff::Buff) },
    };

    writer.write_byte(b'W');
    writer.write_string("e v'e ");
    writer.write_string("been fucked up!");

    loop {}
}
