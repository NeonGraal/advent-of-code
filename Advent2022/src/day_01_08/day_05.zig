const std = @import("std");

const day = "05";

fn read(_: anytype) !?u64 {
    return null;
}

const u8List = std.ArrayList(u8);
const Allocator = std.mem.Allocator;

const State = struct {
    stacks: [9]u8List,
    count: usize,

    fn init(r: anytype, allocator: Allocator) !State {
        var input: [9]u8List = undefined;
        for (input) |*item| item.* = u8List.init(allocator);
        defer for (input) |item| item.deinit();

        var count: usize = 0;
        var buf: [100]u8 = undefined;
        while (try r.readUntilDelimiterOrEof(&buf, '\n')) |line| {
            if (line[1] == '1') {
                count = line.len / 4;
                break;
            }

            var i: u8 = 0;
            while (i * 4 < line.len) : (i += 1) {
                var char = line[i * 4 + 1];
                if (char != ' ') {
                    try input[i].append(char);
                }
            }
        }

        var result = State{ .stacks = undefined, .count = count };

        for (input) |*list, i| {
            result.stacks[i] = u8List.init(allocator);
            while (list.popOrNull()) |item| try result.stacks[i].append(item);
        }

        return result;
    }

    fn deinit(self: State) void {
        for (self.stacks) |item| item.deinit();
    }

    pub fn format(
        self: State,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        var i: usize = 0;
        while (i < self.count) : (i += 1) {
            try writer.print("{d}: ", .{i});
            for (self.stacks[i].items) |item| try writer.print(" [{c}]", .{item});
            try writer.writeAll("\n");
        }
    }
};

fn part1(name: []const u8, allocator: Allocator) ![10:0]u8 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var r = file.reader();

    var state = try State.init(r, allocator);
    defer state.deinit();

    std.debug.print("{s}", .{state});

    var result: [10:0]u8 = undefined;
    return result;
}

fn part2(name: []const u8, allocator: Allocator) ![10:0]u8 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    _ = allocator;

    var result: [10:0]u8 = undefined;
    while (try read(file.reader())) |_| {}
    return result;
}

const test_allocator = std.testing.allocator;

test "part 1 test" {
    const expected: [:0]const u8 = "CMZ";
    const result = try part1("sample/day_" ++ day ++ ".txt", test_allocator);
    try std.testing.expectEqualStrings(expected, &result);
}

test "part 2 test" {
    const expected: []const u8 = "";
    const result = try part2("sample/day_" ++ day ++ ".txt", test_allocator);
    try std.testing.expectEqualStrings(expected, &result);
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result1 = try part1("input/day_" ++ day ++ ".txt", allocator);
    try stdout.print("Part 1 - {s}\n", .{result1});

    const result2 = try part2("input/day_" ++ day ++ ".txt", allocator);
    try stdout.print("Part 2 - {s}\n", .{result2});
}
