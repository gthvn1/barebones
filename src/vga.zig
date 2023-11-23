//! # VGA Text Mode
//!
//! https://en.wikipedia.org/wiki/VGA_text_mode
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

pub fn TextMode() type {
    return struct {
        const VGA_ADDR = 0xB8000;
        const VGA_WIDTH = 80;
        const VGA_HEIGHT = 25;
        const VGA_SIZE = VGA_WIDTH * VGA_HEIGHT;

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

        pub fn putInt(self: *Self, i: u32) void {
            // u32 as at most 10 characters.
            var number: u32 = i;
            var str_number: [16]u8 = undefined;
            var idx: usize = 0;

            // str will be reversed
            while (true) : (idx += 1) {
                const digit: u8 = @truncate(number % 10);
                str_number[idx] = digit + '0';
                number = number / 10;
                if (number == 0) break;
            }

            // print starting from end
            while (true) : (idx -= 1) {
                self.putChar(str_number[idx]);
                if (idx == 0) break;
            }
        }

        pub fn putChar(self: *Self, char: u8) void {
            // Deal with special characters
            switch (char) {
                '\n' => self.row += 1,
                '\r' => self.column = 0,
                else => {
                    const idx: usize = self.row * VGA_WIDTH + self.column;
                    self.buffer[idx] = vgaEntry(char, self.color);
                    self.column += 1;
                },
            }

            // Check boudaries
            if (self.column == VGA_WIDTH) {
                self.column = 0;
                self.row += 1;
            }

            if (self.row == VGA_HEIGHT) {
                self.row = 0;
            }
        }

        pub fn putString(self: *Self, str: []const u8) void {
            for (0..str.len) |i|
                self.putChar(str[i]);
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
