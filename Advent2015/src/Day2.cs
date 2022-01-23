namespace Advent2015;

public class Day2 : DayOfAdvent<Day2>, IDayOfAdvent
{
  record struct Dim(int l, int w, int h)
  {
    static public Dim Parse(string line)
    {
      var parts = line.Split('x').Select(int.Parse).ToArray();
      return new Dim(parts[0], parts[1], parts[2]);
    }

    internal int WrappingArea()
    {
      var areas = new[] { l * w, l * h, h * w };
      var min = areas.Min();

      return areas.Sum() * 2 + min;
    }

    internal int RibbonLength()
    {
      var areas = new[] { l + w, l + h, h + w };
      var min = areas.Min();

      return min * 2 + l * w * h;
    }
  }

  Dim[] ParseInput() =>
    Lines().Select(Dim.Parse).ToArray();

  public int Part1() =>
    ParseInput().Sum(d => d.WrappingArea());
  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() =>
    ParseInput().Sum(d => d.RibbonLength());
  public string Part2Result() =>
    $"{Part2()}";
}
