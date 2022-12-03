const std = @import("std");

const day = "03";

fn read1(r: anytype) !?u64 {
    var buf: [1024]u8 = undefined;
    const line = try r.readUntilDelimiterOrEof(&buf, '\n') orelse return null;
    const elf = std.mem.trimRight(u8, line, "\r\n");

    var seen: [53]bool = .{false} ** 53;
    var size = elf.len / 2;
    for (elf) |item, i| {
        var pri = priority(item);
        if (i < size) {
            seen[pri] = true;
        } else if (seen[pri]) {
            return pri;
        }
    }

    return 0;
}

fn read2(r: anytype) !?u64 {
    var buf1: [1024]u8 = undefined;
    const line1 = try r.readUntilDelimiterOrEof(&buf1, '\n') orelse return null;
    const elf1 = std.mem.trimRight(u8, line1, "\r\n");

    var buf2: [1024]u8 = undefined;
    const line2 = try r.readUntilDelimiterOrEof(&buf2, '\n') orelse return null;
    const elf2 = std.mem.trimRight(u8, line2, "\r\n");

    var buf3: [1024]u8 = undefined;
    const line3 = try r.readUntilDelimiterOrEof(&buf3, '\n') orelse return null;
    const elf3 = std.mem.trimRight(u8, line3, "\r\n");

    var seen: [53]u8 = .{0} ** 53;

    for (elf1) |item| seen[priority(item)] = 1;
    for (elf2) |item| seen[priority(item)] |= 2;
    for (elf3) |item| {
        const pri = priority(item);
        if (seen[pri] == 3) {
            return pri;
        }
    }

    return 0;
}

fn priority(item: u8) u8 {
    return switch (item) {
        'a'...'z' => item - 'a' + 1,
        'A'...'Z' => item - 'A' + 27,
        else => 0,
    };
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
    try std.testing.expectEqual(@intCast(u64, 157), result);
}

test "part 2 test" {
    const result = try part2("sample/day_" ++ day ++ ".txt");
    try std.testing.expectEqual(@intCast(u64, priority('r') + priority('Z')), result);
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    const result1 = try part1("input/day_" ++ day ++ ".txt");
    try stdout.print("Part 1 - {}\n", .{result1});

    const result2 = try part2("input/day_" ++ day ++ ".txt");
    try stdout.print("Part 2 - {}\n", .{result2});
}
