namespace Advent2015;

public class Day9 : DayOfAdvent<Day9>, IDayOfAdvent
{
  readonly Dictionary<string, int> dists = new();
  readonly HashSet<string> locs = new();

  record struct Trip(int dist, string at, HashSet<string> remaining);

  readonly List<Trip> trips = new();

  void Parse(string line) {
    var parts = line.Split(' ');

    dists[$"{parts[0]}->{parts[2]}"] = int.Parse(parts[4]);
    dists[$"{parts[2]}->{parts[0]}"] = int.Parse(parts[4]);
    locs.Add(parts[0]);
    locs.Add(parts[2]);
  }

  HashSet<string> OnWard(HashSet<string> curr, string next)
    => curr.Where(s => s != next).ToHashSet();

  public int Part1() {
    FirstTrips();

    var least = trips.OrderBy(t => t.dist).First();

    while (least.remaining.Count() > 0) {
      trips.Remove(least);
      foreach (var next in least.remaining) {
        var leg = $"{least.at}->{next}";
        if (dists.TryGetValue(leg, out var dist)) {
          trips.Add(new Trip(least.dist + dist, next, OnWard(least.remaining, next)));
        }
      }
      least = trips.OrderBy(t => t.dist).First();
    }

    return least.dist;
  }

  void FirstTrips() {
    foreach (var line in Lines()) {
      Parse(line);
    }

    foreach (var start in locs) {
      var onward = OnWard(locs, start);
      foreach (var to in onward) {
        var leg = $"{start}->{to}";
        if (dists.TryGetValue(leg, out var dist)) {
          trips.Add(new Trip(dist, to, OnWard(onward, to)));
        }
      }
    }
  }

  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() {
    FirstTrips();

    var curr = trips.ToArray();
    List<Trip> completed = new();

    while (curr.Any()) {
      trips.Clear();
      foreach (var c in curr) {
        if (c.remaining.Any()) {
          foreach (var next in c.remaining) {
            var leg = $"{c.at}->{next}";
            if (dists.TryGetValue(leg, out var dist)) {
              trips.Add(new Trip(c.dist + dist, next, OnWard(c.remaining, next)));
            }
          }
        } else {
          completed.Add(c);
        }
      }
      curr = trips.ToArray();
    }

    return completed.Select(c => c.dist).Max();
  }

  public string Part2Result() =>
    $"{Part2()}";
}
