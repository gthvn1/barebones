const std = @import("std");

pub fn build(b: *std.Build) void {
    var target = b.standardTargetOptions(.{});
    target.cpu_model = .native;
    target.cpu_arch = .x86; // 32 bits
    target.os_tag = .freestanding; // Otherwise it will look for a main function

    const optimize = b.standardOptimizeOption(.{});

    const kernel = b.addExecutable(.{
        .name = "myos.bin",
        .root_source_file = .{ .path = "src/kernel.zig" },
        .target = target,
        .optimize = optimize,
    });

    kernel.addAssemblyFile(std.Build.LazyPath{ .path = "src/boot.s" });
    kernel.setLinkerScript(.{ .path = "linker.ld" });

    b.installArtifact(kernel);

    const run_cmd = b.addSystemCommand(&.{ "qemu-system-i386", "-kernel" });
    run_cmd.addArtifactArg(kernel);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args|
        run_cmd.addArgs(args);

    const run_step = b.step("run", "Build myos.bin");
    run_step.dependOn(&run_cmd.step);
}
