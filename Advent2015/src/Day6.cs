namespace Advent2015;

public class Day6 : DayOfAdvent<Day6>, IDayOfAdvent {
  public record struct Rect(int Left, int Top, int Right, int Bottom) {
    public IEnumerable<(int x, int y)> Sweep() {
      for (var x = Left; x <= Right; x++)
        for (var y = Top; y <= Bottom; y++)
          yield return (x, y);
    }
  }

  public enum Action { On, Off, Toggle };

  public record struct Act(Action Action, Rect Rect) {
    static public Act Parse(string line) {
      var parts = line.Split(' ');
      if (parts[0] == "turn") parts = parts[1..];
      var action = Enum.Parse<Action>(parts[0], true);
      var lt = parts[1].Split(',').Select(int.Parse).ToArray();
      var rb = parts[3].Split(',').Select(int.Parse).ToArray();

      var (left, right) = lt[0] < rb[0] ? (lt[0], rb[0]) : (rb[0], lt[0]);
      var (top, bottom) = lt[1] < rb[1] ? (lt[1], rb[1]) : (rb[1], lt[1]);

      return new Act(action, new Rect(left, top, right, bottom));
    }
  }

  readonly bool[][] lights = Enumerable.Range(0, 1000).Select(i => new bool[1000]).ToArray();

  public void Apply1(Act act) {
    Func<bool, bool> change = act.Action switch {
      Action.On => (l) => true,
      Action.Off => (l) => false,
      Action.Toggle => (l) => !l,
      _ => throw new NotImplementedException(),
    };

    foreach (var (x, y) in act.Rect.Sweep())
      lights[x][y] = change(lights[x][y]);
  }

  public int Part1() {
    foreach (var act in Lines().Select(Act.Parse))
      Apply1(act);
    return lights.SelectMany(l => l).Count(l => l);
  }

  public string Part1Result() =>
    $"{Part1()}";

  readonly int[][] brights = Enumerable.Range(0, 1000).Select(i => new int[1000]).ToArray();

  public void Apply2(Act act) {
    Func<int, int> change = act.Action switch {
      Action.On => (b) => b + 1,
      Action.Off => (b) => b > 0 ? b - 1 : 0,
      Action.Toggle => (b) => b + 2,
      _ => throw new NotImplementedException(),
    };

    foreach (var (x, y) in act.Rect.Sweep())
      brights[x][y] = change(brights[x][y]);
  }

  public int Part2() {
    foreach (var act in Lines().Select(Act.Parse))
      Apply2(act);
    return brights.SelectMany(l => l).Sum(l => l);
  }
  public string Part2Result() =>
    $"{Part2()}";
}
