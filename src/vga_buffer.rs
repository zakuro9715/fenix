use core::fmt;
use volatile::Volatile;

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
pub struct Colors(u8);

impl Colors {
    fn new(foreground: Color, background: Color) -> Colors {
        Colors((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color: Colors,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [Volatile<ScreenChar>; BUFFER_WIDTH * BUFFER_HEIGHT],
}

pub struct Writer {
    row: usize,
    col: usize,
    color: Colors,
    buffer: &'static mut Buffer,
}

fn calc_buffer_index(row: usize, col: usize) -> usize {
    row * BUFFER_WIDTH + col
}

impl Writer {
    pub fn new(color: Colors) -> Self {
        Self {
            row: 0,
            col: 0,
            color,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn new_line(&mut self) {
        self.row += 1;
        self.col = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii: b' ',
            color: self.color,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[calc_buffer_index(row, col)].write(blank);
        }
    }

    fn put_byte(&mut self, byte: u8) {
        self.buffer.chars[calc_buffer_index(self.row, self.col)].write(ScreenChar {
            ascii: byte,
            color: self.color,
        });
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.put_byte(byte);
                self.col += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(match byte {
                // printable characters
                0x20..=0x7e | b'\n' => byte,
                // replace unprintable characters with a space
                _ => b' ',
            })
        }
    }

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
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

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> =
        Mutex::new(Writer::new(Colors::new(Color::Yellow, Color::Black),));
}

#[test_case]
fn test_println() {
    let s1 = "test println";
    let s2 = "next line";
    let row = WRITER.lock().row;
    println!("{}\n{}", s1, s2);
    for (i, expected) in s1.chars().enumerate() {
        let actual = char::from(WRITER.lock().buffer.chars[calc_buffer_index(row, i)].read().ascii);
        assert_eq!(expected, actual);
    }
    for (i, expected) in s2.chars().enumerate() {
        let actual = char::from(WRITER.lock().buffer.chars[calc_buffer_index(row + 1, i)].read().ascii);
        assert_eq!(expected, actual);
    }
}
