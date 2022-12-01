const std = @import("std");

pub fn main() !void {
    const max = try maxGroup("input/day_01.txt");

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Part 1 - {}\n", .{max});
}

fn maxGroup(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var max: u64 = 0;
    var group: u64 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const trimmed = std.mem.trimRight(u8, line, "\r\n");
        if (trimmed.len == 0) {
            if (group > max) {
                max = group;
            }
            group = 0;
        } else {
            const val: u64 = try std.fmt.parseInt(u64, trimmed, 10);
            group += val;
        }
    }
    return max;
}

test "simple test" {
    const max = try maxGroup("sample/day_01.txt");
    try std.testing.expectEqual(@intCast(u64, 24000), max);
}
