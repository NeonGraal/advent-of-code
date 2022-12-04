const std = @import("std");

const day = "04";

fn read(_: anytype) !?u64 {
    return null;
}

fn part1(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var result: u64 = 0;
    while (try read(file.reader())) |group| {
        result += group;
    }
    return result;
}

fn part2(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var result: u64 = 0;
    while (try read(file.reader())) |group| {
        result += group;
    }
    return result;
}

test "part 1 test" {
    const result = try part1("sample/day_" ++ day ++ ".txt");
    try std.testing.expectEqual(@intCast(u64, 0), result);
}

test "part 2 test" {
    const result = try part2("sample/day_" ++ day ++ ".txt");
    try std.testing.expectEqual(@intCast(u64, 0), result);
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    const result1 = try part1("input/day_" ++ day ++ ".txt");
    try stdout.print("Part 1 - {}\n", .{result1});

    const result2 = try part2("input/day_" ++ day ++ ".txt");
    try stdout.print("Part 2 - {}\n", .{result2});
}
