namespace Advent2015;

public class Day24 : DayOfAdvent<Day24>, IDayOfAdvent
{
  record struct Groups(int[] group1, int[] group2, int sum1, int sum2)
  {
    public IEnumerable<Groups> Add(int package, int weight) {
      if (sum1 + package <= weight) {
        yield return new Groups(group1.Append(package).ToArray(), group2, sum1 + package, sum2);
      }
      if (sum2 + package <= weight) {
        yield return new Groups(group1, group2.Append(package).ToArray(), sum1, sum2 + package);
      }
    }

    public bool Contains(int package) =>
      group1.Contains(package) || group2.Contains(package);

    public bool Full(int weight) =>
      sum1 == weight && sum2 == weight && group1.Length <= group2.Length;
  }

  Groups[] Balance(int[] packages, int groups) {
    var weight = packages.Sum() / groups;
    var result = new Groups(Array.Empty<int>(), Array.Empty<int>(), 0, 0);

    foreach (var p in packages.OrderByDescending(p => p)) {
      if (result.sum1 + p <= weight && result.sum2 + p <= weight) {
        result = result.Add(p, weight).First();
      }
    }

    return new[] { result };
  }

  public long Part1() {
    var packages = Lines().ToInts(0).ToArray();

    var groups = Balance(packages, 3);

    return groups.Select(g => g.group1.Aggregate(1L, (t, i) => t * i)).Min();
  }
  public string Part1Result() =>
    $"{Part1()}";

  public long Part2() {
    var packages = Lines().ToInts(0).ToArray();

    var groups = Balance(packages, 4);

    return groups.Select(g => g.group1.Aggregate(1L, (t, i) => t * i)).Min();
  }

  // 349248535 too high
  public string Part2Result() =>
    $"{Part2()}";
}