const std = @import("std");

const day = "04";

fn read1(r: anytype) !?u64 {
    var low_1 = try readAndParseUntil(r, u8, '-') orelse return null;
    var high_1 = try readAndParseUntil(r, u8, ',') orelse return null;
    var low_2 = try readAndParseUntil(r, u8, '-') orelse return null;
    var high_2 = try readAndParseUntil(r, u8, '\n') orelse return null;

    if (low_2 <= low_1 and high_2 >= high_1) return 1;
    if (low_2 >= low_1 and high_2 <= high_1) return 1;
    return 0;
}

fn read2(r: anytype) !?u64 {
    var low_1 = try readAndParseUntil(r, u8, '-') orelse return null;
    var high_1 = try readAndParseUntil(r, u8, ',') orelse return null;
    var low_2 = try readAndParseUntil(r, u8, '-') orelse return null;
    var high_2 = try readAndParseUntil(r, u8, '\n') orelse return null;

    if (high_1 < low_2 or high_2 < low_1) return 0;
    return 1;
}

fn readAndParseUntil(r: anytype, comptime T: type, delimiter: u8) !?T {
    var buf: [8]u8 = undefined;
    const input = try r.readUntilDelimiterOrEof(&buf, delimiter) orelse return null;
    const trimmed = std.mem.trimRight(u8, input, "\r\n");
    if (trimmed.len < 1) return null;
    return try std.fmt.parseInt(T, trimmed, 10);
}

fn part1(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var result: u64 = 0;
    while (try read1(file.reader())) |group| {
        result += group;
    }
    return result;
}

fn part2(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var result: u64 = 0;
    while (try read2(file.reader())) |group| {
        result += group;
    }
    return result;
}

test "part 1 test" {
    const result = try part1("sample/day_" ++ day ++ ".txt");
    try std.testing.expectEqual(@intCast(u64, 2), result);
}

test "part 2 test" {
    const result = try part2("sample/day_" ++ day ++ ".txt");
    try std.testing.expectEqual(@intCast(u64, 4), result);
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    const result1 = try part1("input/day_" ++ day ++ ".txt");
    try stdout.print("Part 1 - {}\n", .{result1});

    const result2 = try part2("input/day_" ++ day ++ ".txt");
    try stdout.print("Part 2 - {}\n", .{result2});
}
