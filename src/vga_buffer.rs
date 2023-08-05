use core::{
    fmt,
    ptr::{read_volatile, write_volatile},
};
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub px: usize,
    pub c: ColorCode,
    pub buf: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.px >= BUFFER_WIDTH {
                    self.new_line();
                }
                let y = BUFFER_HEIGHT - 1;
                let x = self.px;
                unsafe {
                    write_volatile(
                        &mut self.buf.chars[y][x],
                        ScreenChar {
                            ascii_character: byte,
                            color_code: self.c,
                        },
                    );
                }
                self.px += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for y in 0..BUFFER_HEIGHT - 1 {
            for x in 0..BUFFER_WIDTH {
                unsafe {
                    write_volatile(
                        &mut self.buf.chars[y][x],
                        read_volatile(&mut self.buf.chars[y + 1][x]),
                    )
                }
            }
        }
        self.clear_line();
        self.px = 0;
    }

    pub fn clear_line(&mut self) {
        self.px = 0;
        for i in 0..BUFFER_WIDTH {
            unsafe {
                write_volatile(
                    &mut self.buf.chars[BUFFER_HEIGHT - 1][i],
                    ScreenChar {
                        ascii_character: b' ',
                        color_code: self.c,
                    },
                )
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            match b {
                0x20..=0x7e | b'\n' => self.write_byte(b),
                _ => self.write_byte(0xfe),
            };
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        px: 0,
        c: ColorCode::new(Color::Green, Color::Black),
        buf: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = unsafe{read_volatile(&WRITER.lock().buf.chars[BUFFER_HEIGHT - 2][i])};
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}