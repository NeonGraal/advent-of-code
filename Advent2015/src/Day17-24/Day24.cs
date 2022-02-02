namespace Advent2015;

public class Day24 : DayOfAdvent<Day24>, IDayOfAdvent
{
  IEnumerable<int[]> Balance(int[] packages, int groups) {
    var weight = packages.Sum() / groups;
    var possibles = new List<int[]> { Array.Empty<int>() };

    foreach (var p in packages.OrderByDescending(p => p)) {
      foreach (var g in possibles.ToArray()) {
        var sum = g.Sum() + p;
        if (sum > weight) { continue; }
        var next = g.Append(p).ToArray();
        if (sum == weight) { yield return next; } else { possibles.Add(next); }
      }
    }
  }

  public long Part1() {
    var packages = Lines().ToInts(0).ToArray();

    var groups = Balance(packages, 3);

    var smallest = groups.GroupBy(g => g.Length).MinBy(g => g.Key)?.ToArray() ?? Array.Empty<int[]>();

    return smallest.Select(g => g.Aggregate(1L, (t, i) => t * i)).Min();
  }
  public string Part1Result() =>
    $"{Part1()}";

  public long Part2() {
    var packages = Lines().ToInts(0).ToArray();

    var groups = Balance(packages, 4);

    var smallest = groups.GroupBy(g => g.Length).MinBy(g => g.Key)?.ToArray() ?? Array.Empty<int[]>();

    return smallest.Select(g => g.Aggregate(1L, (t, i) => t * i)).Min();
  }

  // 349248535 too high
  public string Part2Result() =>
    $"{Part2()}";
}