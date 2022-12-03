const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    const games = try totalGames("input/day_02.txt");
    try stdout.print("Part 1 - {}\n", .{games});
    // Wrong Answers:
    // 14858 too high
    // 13022 too low

    const strategies = try totalStrategies("input/day_02.txt");
    try stdout.print("Part 2 - {}\n", .{strategies});
}

fn totalGames(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var total: u64 = 0;

    var r = file.reader();
    var buf: [1024]u8 = undefined;
    while (try r.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        total += scoreGame(line[0], line[2]);
    }

    return total;
}

fn totalStrategies(name: []const u8) !u64 {
    var file = try std.fs.cwd().openFile(name, .{});
    defer file.close();

    var total: u64 = 0;

    var r = file.reader();
    var buf: [1024]u8 = undefined;
    while (try r.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        total += scoreStrategy(line[0], line[2]);
    }

    return total;
}

fn scoreGame(opponent: u8, player: u8) u64 {
    const mine = player - 'W';
    return @intCast(u64, switch (opponent) {
        'A' => ([4]u64{ 0, 4, 8, 3 })[mine],
        'B' => ([4]u64{ 0, 1, 5, 9 })[mine],
        'C' => ([4]u64{ 0, 7, 2, 6 })[mine],
        else => 0,
    });
}

fn scoreStrategy(opponent: u8, player: u8) u64 {
    const mine = player - 'W';
    return @intCast(u64, switch (opponent) {
        'A' => ([4]u64{ 0, 3, 4, 8 })[mine],
        'B' => ([4]u64{ 0, 1, 5, 9 })[mine],
        'C' => ([4]u64{ 0, 2, 6, 7 })[mine],
        else => 0,
    });
}

test "rock beats scissors" {
    const score = scoreGame('C', 'X');
    try std.testing.expectEqual(@intCast(u64, 7), score);
}

test "rock loses to paper" {
    const score = scoreGame('B', 'X');
    try std.testing.expectEqual(@intCast(u64, 1), score);
}

test "paper beats rock" {
    const score = scoreGame('A', 'Y');
    try std.testing.expectEqual(@intCast(u64, 8), score);
}

test "paper loses to scissors" {
    const score = scoreGame('C', 'Y');
    try std.testing.expectEqual(@intCast(u64, 2), score);
}

test "scissors beats paper" {
    const score = scoreGame('B', 'Z');
    try std.testing.expectEqual(@intCast(u64, 9), score);
}

test "scissors loses to rock" {
    const score = scoreGame('A', 'Z');
    try std.testing.expectEqual(@intCast(u64, 3), score);
}

test "both paper draw" {
    const score = scoreGame('A', 'X');
    try std.testing.expectEqual(@intCast(u64, 4), score);
}

test "both rock draw" {
    const score = scoreGame('B', 'Y');
    try std.testing.expectEqual(@intCast(u64, 5), score);
}

test "both scissors draw" {
    const score = scoreGame('C', 'Z');
    try std.testing.expectEqual(@intCast(u64, 6), score);
}

test "part 1 test" {
    const total = try totalGames("sample/day_02.txt");
    try std.testing.expectEqual(@intCast(u64, 15), total);
}

test "part 2 test" {
    const total = try totalStrategies("sample/day_02.txt");
    try std.testing.expectEqual(@intCast(u64, 12), total);
}
