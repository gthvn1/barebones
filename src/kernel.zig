const vga = @import("vga_text_mode.zig");

export fn kernel_main() noreturn {
    var tty = vga.VGATextMode(){};
    tty.clear();
    tty.putString("All your codebase are belong to us.");
    while (true) {}
}
