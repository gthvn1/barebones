const vga = @import("vga_text_mode.zig");

export fn kernel_main() noreturn {
    var tty = vga.VGATextMode(){};
    tty.clear();
    tty.putString("All your codebase are belong to us.\n\r");
    tty.putString("We're counting on you, ZIG!!");
    while (true) {}
}
