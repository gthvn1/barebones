const vga = @import("vga_text_mode.zig");

export fn kernel_main() noreturn {
    var tty = vga.VGATextMode(){};
    tty.clear();
    tty.putChar('Z');
    tty.putChar('I');
    tty.putChar('G');

    while (true) {}
}
