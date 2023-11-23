const terminal = @import("terminal.zig");

export fn kernel_main() noreturn {
    var tty1 = terminal.tty{};
    tty1.clear();

    while (true) {}
}
