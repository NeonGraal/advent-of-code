namespace Advent2015;

public class Day14 : DayOfAdvent<Day14>, IDayOfAdvent {
  record Reindeer(string Name, int Speed, int Fly, int Rest) {
    public static Reindeer Parse(string line) {
      var parts = line.Split(' ');
      var nums = parts.Select(p => int.TryParse(p, out var v) ? v : -1).ToArray();

      return new(parts[0], nums[3], nums[6], nums[13]);
    }

    public int Score { get; private set; } = 0;

    public void Leader() => Score++;

    public int Distance(int seconds) {
      var period = Fly + Rest;
      var complete = seconds / period;
      var remain = seconds % period;
      if (remain == 0) {
        return complete * Fly * Speed;
      }
      if (remain < Fly) {
        return (complete * Fly + remain) * Speed;
      }
      return (complete + 1) * Fly * Speed;
    }
  }

  public int Part1(int duration) =>
    Lines().Select(Reindeer.Parse).Select(r => r.Distance(duration)).Max();
  public string Part1Result() =>
    $"{Part1(2503)}";

  public int Part2(int duration) {
    var reindeer = Lines().Select(Reindeer.Parse).ToArray();

    for (var i = 1; i <= duration; i++) {
      foreach (var l in reindeer.GroupBy(r => r.Distance(i)).MaxBy(g => g.Key)) {
        l.Leader();
      }
    }

    return reindeer.Select(r => r.Score).Max();
  }

  public string Part2Result() =>
    $"{Part2(2503)}";
}
