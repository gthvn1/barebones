// VGA Text Mode Driver
//
// Reference: https://en.wikipedia.org/wiki/VGA_text_mode
// Reference: https://os.phil-opp.com/vga-text-mode/
//
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
#[repr(C)]
struct Char {
    ascii_char: u8,
    color_code: u8,
}

fn get_color_code(foreground: Color, background: Color) -> u8 {
    (background as u8) << 4 | (foreground as u8)
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

struct TextMode {
    row_position: usize,
    column_position: usize,
    color_code: u8,
    buffer: &'static mut Buffer, // Constants must be explicitly typed. The
                                 // type must have a 'static lifetime
}

impl TextMode {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.row_position += 1,
            b'\r' => self.column_position = 0,
            _ => {
                self.buffer.chars[self.row_position][self.column_position] = Char {
                    ascii_char: byte,
                    color_code: self.color_code,
                };

                self.column_position += 1;
            }
        }

        // Check boundaries after upgrading position.
        if self.column_position >= BUFFER_WIDTH {
            self.column_position = 0;
            self.row_position += 1;
        }

        // TODO: handle scrolling.
        if self.row_position >= BUFFER_HEIGHT {
            self.row_position = 0;
        }
    }
}

const BANNER: &[u8] = b"Enter the Enchanted Shores of Simian Atoll!";

pub fn banner() {
    let mut writer = TextMode {
        row_position: 0,
        column_position: 0,
        color_code: get_color_code(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    for &byte in BANNER.iter() {
        writer.write_byte(byte);
    }
}
