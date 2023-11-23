const vga = @import("vga.zig");
const builtin = @import("std").builtin;

var tty = vga.TextMode(){};

pub fn panic(msg: []const u8, err: ?*builtin.StackTrace, s: ?usize) noreturn {
    _ = s;
    _ = err;
    tty.setColor(.red, .white);
    tty.putString(msg);
    while (true) {}
}

export fn kernel_main() noreturn {
    tty.clear();
    tty.putString("All your codebase are belong to us.\n\r");
    tty.putString("We're counting on you, ZIG!!\n\r");
    tty.putInt(42);

    @panic("This is the end, my only friend...");
}
