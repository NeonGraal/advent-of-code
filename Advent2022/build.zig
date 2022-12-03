const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();

    setup_day(b, target, mode, "day_01_08", "01");
    setup_day(b, target, mode, "day_01_08", "02");
    setup_day(b, target, mode, "day_01_08", "03");
}

fn setup_day(b: *std.build.Builder, target: std.zig.CrossTarget, mode: std.builtin.Mode, comptime dir: []const u8, comptime day: []const u8) void {
    const source = "src/" ++ dir ++ "/day_" ++ day ++ ".zig";

    const exe = b.addExecutable("Advent2022_day_" ++ day, source);
    exe.setTarget(target);
    exe.setBuildMode(mode);
    exe.install();
    const run_cmd = exe.run();
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run_" ++ day, "Run challenge for day " ++ day);
    run_step.dependOn(&run_cmd.step);

    const exe_tests = b.addTest(source);
    exe_tests.setTarget(target);
    exe_tests.setBuildMode(mode);
    const test_step = b.step("test_" ++ day, "Run unit tests for day " ++ day);
    test_step.dependOn(&exe_tests.step);
}
