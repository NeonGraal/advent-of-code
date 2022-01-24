namespace Advent2015;

public interface IDayOfAdvent {
  string DayName { get; }
  void LoadInput();
  string Part1Result();
  string Part2Result();
}