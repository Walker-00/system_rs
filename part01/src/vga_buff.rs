#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_char: u8,
    pub color_code: ColorCode,
}

const BUFF_HGT: usize = 25;
const BUFF_WIT: usize = 80;

#[repr(transparent)]
pub struct Buff {
    pub chars: [[ScreenChar; BUFF_WIT]; BUFF_HGT],
}

pub struct Writer {
    pub clmn_pst: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buff,
}

impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.clmn_pst >= BUFF_WIT {
                    self.new_line();
                }

                let row = BUFF_HGT - 1;
                let col = self.clmn_pst;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code,
                };

                self.clmn_pst += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {}
}

pub fn write_some() {
    let mut writer = Writer {
        clmn_pst: 0,
        color_code: ColorCode::new(Color::Cyan, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buff) },
    };

    writer.write_byte(b'W');
    writer.write_string("e've ");
    writer.write_string("been Fucked up!");
}
