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

pub const tty = struct {
    const VGA_WIDHT = 80;
    const VGA_HEIGHT = 25;
    const VGA_SIZE = VGA_WIDHT * VGA_HEIGHT;

    row: usize = 0,
    column: usize = 0,
    color: VGAColor = .white,
    buffer: [*]volatile u16 = @as([*]volatile u16, @ptrFromInt(0xB8000)),

    pub fn clear(self: *tty) void {
        for (0..VGA_SIZE) |i| {
            self.buffer[i] = 0;
        }
    }
};
