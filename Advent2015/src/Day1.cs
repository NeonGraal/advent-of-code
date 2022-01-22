namespace Advent2015;

public class Day1 : DayOfAdvent<Day1>, IDayOfAdvent
{
  static int AdjustFloor(int total, char dir) =>
    dir switch
    {
      '(' => total + 1,
      ')' => total - 1,
      _ => total
    };

  public int Part1() =>
    _input.Aggregate(0, AdjustFloor);
  public string Part1Result() =>
    $"{Part1()}";

  public int Part2()
  {
    var total = 0;
    for (var i = 0; i < _input.Length; i++)
    {
      total = AdjustFloor(total, _input[i]);
      if (total < 0)
        return i + 1;
    }
    return -1;
  }

  public string Part2Result() =>
    $"{Part2()}";
}
