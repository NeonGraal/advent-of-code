namespace Advent2015;

public class Day18 : DayOfAdvent<Day18>, IDayOfAdvent
{
  static bool CellAlive(bool cell, params bool[] neighbors) =>
    cell ? neighbors.Count(n => n) is 2 or 3 : neighbors.Count(n => n) == 3;

  static bool[][] Generation(bool[][] board) {
    var first = board[0];
    var width = first.Length;
    var blank = first.Select(b => false).ToArray();
    var above = blank.Append(false).Prepend(false).ToArray();
    var curr = first.Append(false).Prepend(false).ToArray();
    var results = new List<bool[]>();

    foreach (var line in board[1..].Append(blank)) {
      var below = line.Append(false).Prepend(false).ToArray();

      var result = new bool[width];
      for (var i = 0; i < width; i++) {
        result[i] = CellAlive(curr[i + 1], curr[i], curr[i + 2], above[i], above[i + 1], above[i + 2], below[i], below[i + 1], below[i + 2]);
      }
      results.Add(result);

      (above, curr) = (curr, below);
    }

    return results.ToArray();
  }

  public int Part1(int rounds) {
    var board = Lines().Select(l => l.Select(c => c == '#').ToArray()).ToArray();

    for (var i = 0; i < rounds; i++) {
      board = Generation(board);
    }

    return board.SelectMany(b => b).Count(b => b);
  }
  public string Part1Result() =>
    $"{Part1(100)}";

  public int Part2(int rounds) {
    var board = Lines().Select(l => l.Select(c => c == '#').ToArray()).ToArray();
    var right = board[0].Length - 1;
    var bottom = board.Length - 1;
    board[0][0] = board[bottom][0] = board[0][right] = board[bottom][right] = true;

    for (var i = 0; i < rounds; i++) {
      board = Generation(board);
      board[0][0] = board[bottom][0] = board[0][right] = board[bottom][right] = true;
    }

    return board.SelectMany(b => b).Count(b => b);
  }

  public string Part2Result() =>
    $"{Part2(100)}";
}
