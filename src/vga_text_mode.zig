//! # VGA Text Mode
//!
//! Manage VGA Text buffer. Each screen character is represented by two
//! bytes aligned as a 16-bit word.
//!
//! +-------------------------------+-------------------------------+
//! |         Attribute             |          Character            |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |   |Background |  Foreground   |          Code Point           |
//! +---+-----------+---------------+---+---+---+---+---+---+---+---+
//!   ^
//!   +-- Blink (depending of mode)
//!

pub const VGAColor = enum(u4) {
    black,
    blue,
    green,
    cyan,
    red,
    magenta,
    brown,
    lightGrey,
    darkGray,
    lightBlue,
    lightGreen,
    lightCyan,
    lightRed,
    lightMagenta,
    lightBrown,
    white,
};

pub fn VGATextMode() type {
    return struct {
        const VGA_ADDR = 0xB8000;
        const VGA_WIDHT = 80;
        const VGA_HEIGHT = 25;
        const VGA_SIZE = VGA_WIDHT * VGA_HEIGHT;

        row: usize = 0,
        column: usize = 0,
        color: u8 = vgaEntryColor(.lightGreen, .black),
        buffer: [*]volatile u16 = @as([*]volatile u16, @ptrFromInt(VGA_ADDR)),

        const Self = @This();

        fn vgaEntryColor(fg: VGAColor, bg: VGAColor) u8 {
            return @as(u8, @intFromEnum(fg)) | @as(u8, @intFromEnum(bg)) << 4;
        }

        fn vgaEntry(char: u8, color: u8) u16 {
            return char | (@as(u16, color) << 8);
        }

        pub fn setColor(self: *Self, fg: VGAColor, bg: VGAColor) void {
            self.color = vgaEntryColor(fg, bg);
        }

        pub fn putChar(self: *Self, char: u8) void {
            const idx: usize = self.row * VGA_WIDHT + self.column;
            self.buffer[idx] = vgaEntry(char, self.color);
            self.column += 1;
            if (self.column == VGA_WIDHT) {
                self.column = 0;
                self.row += 1;
            }
            if (self.row == VGA_HEIGHT) {
                self.row = 0;
            }
        }

        pub fn clear(self: *Self) void {
            for (0..VGA_SIZE) |i| {
                self.buffer[i] = vgaEntry(' ', self.color);
            }
            self.row = 0;
            self.column = 0;
        }
    };
}
