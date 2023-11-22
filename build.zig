const std = @import("std");

pub fn build(b: *std.Build) void {
    var target = b.standardTargetOptions(.{});
    target.os_tag = .freestanding;
    target.cpu_arch = .x86;

    const optimize = b.standardOptimizeOption(.{});

    const kernel = b.addExecutable(.{
        .name = "myos.bin",
        .root_source_file = .{ .path = "./src/kernel.zig" },
        .target = target,
        .optimize = optimize,
    });

    kernel.addAssemblyFile(std.Build.LazyPath{ .path = "src/boot.s" });
    kernel.setLinkerScript(.{ .path = "./src/linker.ld" });

    b.installArtifact(kernel);

    const run_cmd = b.addRunArtifact(kernel);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Build myos.bin");
    run_step.dependOn(&run_cmd.step);
}
