namespace Advent2015;

public class Day14 : DayOfAdvent<Day14>, IDayOfAdvent
{
  record Reindeer(string Name, int Speed, int Fly, int Rest)
  {
    public static Reindeer Parse(string line) {
      var parts = line.Split(' ');
      var nums = parts.ToInts(-1);

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
    Lines(Reindeer.Parse).Select(r => r.Distance(duration)).Max();
  public string Part1Result() =>
    $"{Part1(2503)}";

  public int Part2(int duration) {
    var reindeer = Lines(Reindeer.Parse);

    for (var i = 1; i <= duration; i++) {
#pragma warning disable CS8602 // Dereference of a possibly null reference.
      foreach (var l in reindeer.GroupBy(r => r.Distance(i)).MaxBy(g => g.Key)) {
#pragma warning restore CS8602 // Dereference of a possibly null reference.
        l.Leader();
      }
    }

    return reindeer.Select(r => r.Score).Max();
  }

  public string Part2Result() =>
    $"{Part2(2503)}";
}
