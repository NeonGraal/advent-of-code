namespace Advent2015;

public class Day3 : DayOfAdvent<Day3>, IDayOfAdvent
{
  record struct Pos(long x, long y)
  {
    public Pos Step(char c) =>
      c switch
      {
        '>' => new(x + 1, y),
        '<' => new(x - 1, y),
        'v' => new(x, y + 1),
        '^' => new(x, y - 1),
        _ => this
      };
  }

  public int Part1()
  {
    Pos santa = new(0, 0);

    return _input
      .Select(c => santa = santa.Step(c)).Append(new(0, 0))
      .Distinct()
      .Count();
  }
  public string Part1Result() =>
    $"{Part1()}";

  public int Part2()
  {
    Pos santa = new(0, 0);
    var robo = santa;
    return _input.Chunk(2)
      .SelectMany(c =>
      {
        santa = santa.Step(c[0]);
        robo = robo.Step(c[1]);
        return new[] { santa, robo };
      })
      .Append(new(0, 0))
      .Distinct()
      .Count();
  }

  public string Part2Result() =>
    $"{Part2()}";
}
