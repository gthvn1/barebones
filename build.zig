const std = @import("std");

pub fn build(b: *std.Build) void {
    var target: std.zig.CrossTarget = .{
        .cpu_arch = .x86, // 32 bits
        .os_tag = .freestanding, // Otherwise it will look for a main function
        .abi = .none,
    };

    // Disable CPU features that require additional initialization
    // like MMX, SSE/2 and AVX.
    const Features = std.Target.x86.Feature;
    target.cpu_features_sub.addFeature(@intFromEnum(Features.mmx));
    target.cpu_features_sub.addFeature(@intFromEnum(Features.sse));
    target.cpu_features_sub.addFeature(@intFromEnum(Features.sse2));
    target.cpu_features_sub.addFeature(@intFromEnum(Features.avx));
    target.cpu_features_sub.addFeature(@intFromEnum(Features.avx2));
    target.cpu_features_sub.addFeature(@intFromEnum(Features.soft_float));

    const optimize = b.standardOptimizeOption(.{});

    const kernel = b.addExecutable(.{
        .name = "myos.bin",
        .root_source_file = .{ .path = "src/kernel.zig" },
        .target = target,
        .optimize = optimize,
    });

    kernel.addAssemblyFile(std.Build.LazyPath{ .path = "src/boot.s" });
    kernel.setLinkerScript(.{ .path = "linker.ld" });
    kernel.code_model = .kernel;

    b.installArtifact(kernel);

    const run_cmd = b.addSystemCommand(&.{ "qemu-system-i386", "-kernel" });
    run_cmd.addArtifactArg(kernel);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args|
        run_cmd.addArgs(args);

    const run_step = b.step("run", "Build myos.bin");
    run_step.dependOn(&run_cmd.step);
}
