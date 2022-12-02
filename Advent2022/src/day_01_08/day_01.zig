const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    const max = try maxGroup("input/day_01.txt");
    try stdout.print("Part 1 - {}\n", .{max});

    const max3 = try max3Group("input/day_01.txt");
    try stdout.print("Part 2 - {}\n", .{max3});
}

fn readAndSumGroup(r: anytype) !?u64 {
    var group: ?u64 = null;

    var buf: [1024]u8 = undefined;
    while (try r.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const trimmed = std.mem.trimRight(u8, line, "\r\n");
        if (trimmed.len == 0) {
            break;
        } else {
            const val: u64 = try std.fmt.parseInt(u64, trimmed, 10);
            group = val + (group orelse 0);
        }
    }
    return group;
}

fn maxGroup(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var max: u64 = 0;
    while (try readAndSumGroup(file.reader())) |group| {
        if (group > max) {
            max = group;
        }
    }
    return max;
}

fn max3Group(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var max1: u64 = 0;
    var max2: u64 = 0;
    var max3: u64 = 0;
    while (try readAndSumGroup(file.reader())) |group| {
        if (group > max1) {
            max3 = max2;
            max2 = max1;
            max1 = group;
        } else if (group > max2) {
            max3 = max2;
            max2 = group;
        } else if (group > max3) {
            max3 = group;
        }
    }
    return max1 + max2 + max3;
}

test "part 1 test" {
    const max = try maxGroup("sample/day_01.txt");
    try std.testing.expectEqual(@intCast(u64, 24000), max);
}

test "part 2 test" {
    const max = try max3Group("sample/day_01.txt");
    try std.testing.expectEqual(@intCast(u64, 45000), max);
}
