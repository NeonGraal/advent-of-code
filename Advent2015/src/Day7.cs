namespace Advent2015;

public class Day7 : DayOfAdvent<Day7>, IDayOfAdvent
{
  public Dictionary<string, Func<ushort>> Wire { get; } = new();

  ushort GetWire(string wire) {
    if (ushort.TryParse(wire, out var value)) {
      return value;
    }
    var result = Wire[wire].Invoke();
    Wire[wire] = () => result;
    return result;
  }

  void Parse(string line) {
    var parts = line.Split(' ');
    var wire = parts[^1];
    if (parts.Length == 3) {
      Wire[wire] = () => GetWire(parts[0]);
    }
    if (parts.Length == 4) {
      Wire[wire] = () => (ushort)~GetWire(parts[1]);
    }
    if (parts.Length == 5) {
      if (parts[1] == "AND") {
        Wire[wire] = () => (ushort)(GetWire(parts[0]) & GetWire(parts[2]));
      }
      if (parts[1] == "OR") {
        Wire[wire] = () => (ushort)(GetWire(parts[0]) | GetWire(parts[2]));
      }
      if (parts[1] == "LSHIFT") {
        Wire[wire] = () => (ushort)(GetWire(parts[0]) << GetWire(parts[2]));
      }
      if (parts[1] == "RSHIFT") {
        Wire[wire] = () => (ushort)(GetWire(parts[0]) >> GetWire(parts[2]));
      }
    }
  }

  public int Part1(string wire) {
    foreach (var line in Lines()) {
      Parse(line);
    }
    return Wire[wire].Invoke();
  }
  public string Part1Result() =>
    $"{Part1("a")}";

  public int Part2() {
    foreach (var line in Lines()) {
      Parse(line);
    }
    Wire["b"] = () => 16076;
    return Wire["a"].Invoke();
  }

  public string Part2Result() =>
    $"{Part2()}";
}
