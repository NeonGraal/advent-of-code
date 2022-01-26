namespace Advent2015;

public class Day13 : DayOfAdvent<Day13>, IDayOfAdvent {
  (string, string, int) Parse(string line) {
    var parts = line.Split(' ', '.');
    var change = int.Parse(parts[3]);
    if (parts[2] == "lose") {
      change = -change;
    }
    return (parts[0], parts[^2], change);
  }

  Dictionary<(string, string), int> Change = new();

  HashSet<string> Guests = new();

  delegate int Both(string left, string right);

  int BothChange(string left, string right) =>
    Change[(left, right)] + Change[(right, left)];

  record struct Seating(string[] Seats, int Total) {
    public Seating Seat(string guest, Both change) =>
      new Seating(Seats.Append(guest).ToArray(), Total + change(Seats[^1], guest));
    public Seating Close(Both change) {
      Total += change(Seats[^1], Seats[0]);
      return this;
    }
  }

  public int Part1() {
    Change = Lines().Select(Parse).ToDictionary(t => (t.Item1, t.Item2), t => t.Item3);
    Guests = Change.SelectMany(h => new[] { h.Key.Item1, h.Key.Item2 }).ToHashSet();

    return BestHappiness();
  }

  int BestHappiness() {
    var numGuests = Guests.Count();

    var complete = new List<Seating>();
    var seatings = Guests.Select(g => new Seating(new[] { g }, 0)).ToList();
    var incomplete = seatings.ToArray();
    var remaining = Array.Empty<string>();

    while (incomplete.Any()) {
      seatings.Clear();
      foreach (var s in incomplete) {
        remaining = Guests.Except(s.Seats).ToArray();
        if (remaining.Any()) {
          foreach (var g in remaining) {
            seatings.Add(s.Seat(g, BothChange));
          }
        } else {
          complete.Add(s.Close(BothChange));
        }
      }
      incomplete = seatings.ToArray();
    }

    return complete.Select(s => s.Total).Max();
  }

  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() {
    Change = Lines().Select(Parse).ToDictionary(t => (t.Item1, t.Item2), t => t.Item3);
    Guests = Change.SelectMany(h => new[] { h.Key.Item1, h.Key.Item2 }).ToHashSet();

    foreach (var g in Guests) {
      Change[("Me", g)] = 0;
      Change[(g, "Me")] = 0;
    }
    Guests.Add("Me");

    return BestHappiness();
  }

  public string Part2Result() =>
    $"{Part2()}";
}
