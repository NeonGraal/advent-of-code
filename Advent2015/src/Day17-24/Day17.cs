namespace Advent2015;

public class Day17 : DayOfAdvent<Day17>, IDayOfAdvent
{
  record struct Cont(int Size, int Id);
  struct Set
  {
    public Set(Cont container) {
      Containers = new[] { container };
      Size = container.Size;
      Key = $"{container.Id}";
    }

    public Set Extend(Cont next) {
      var containers = Containers.Append(next).OrderBy(c => c.Id).ToArray();
      return new Set() {
        Containers = containers,
        Size = Size + next.Size,
        Key = containers.Show(c => $"{c.Id}")
      };
    }

    public Cont[] Containers { get; private set; }
    public int Size { get; private set; }
    public string Key { get; private set; }
  }

  IEnumerable<Set> Possibles(int litres) {
    var containers = Lines().ToInts(0).Select((s, i) => new Cont(s, i)).ToArray();
    var sets = containers.Select((c, i) => new Set(c)).ToArray();
    var complete = new List<Set>();
    var incomplete = new List<Set>();
    Set next;

    while (sets.Length > 0) {
      incomplete.Clear();
      foreach (var s in sets) {
        foreach (var n in containers.Except(s.Containers)) {
          if (s.Size + n.Size > litres) {
            continue;
          }

          next = s.Extend(n);
          if (next.Size == litres) {
            complete.Add(next);
          } else {
            incomplete.Add(next);
          }
        }
      }
      sets = incomplete.DistinctBy(s => s.Key).ToArray();
    }

    return complete.DistinctBy(s => s.Key);
  }

  public int Part1(int litres) =>
    Possibles(litres).Count();

  public string Part1Result() =>
    $"{Part1(150)}";

  public int Part2(int litres) =>
    Possibles(litres).GroupBy(s => s.Containers.Length).MinBy(g => g.Key)?.Count() ?? 0;

  public string Part2Result() =>
    $"{Part2(150)}";
}
