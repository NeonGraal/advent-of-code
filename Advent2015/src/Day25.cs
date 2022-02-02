namespace Advent2015;

public class Day25 : DayOfAdvent<Day25>, IDayOfAdvent
{
  public record struct Pos(int row, int col);

  public Pos Code { get; set; }

  public new void LoadInput() => Code = new(2981, 3075);

  public long Part1() {
    var top = Code.col * (Code.col + 1) / 2;
    var left = Code.row * (Code.row - 1) / 2 + 1;
    var number = (Code.col + Code.row - 1) * (Code.col + Code.row) / 2 - Code.row + 1;

    Console.WriteLine($"{Code} : T {top} L {left} N {number}");

    var code = 20151125L;

    for (var i = 1; i < number; i++) {
      code = code * 252533 % 33554393;
    }

    return code;
  }
  public string Part1Result() =>
    $"{Part1()}";
}