namespace Advent2015.Test;

public class Day25Tests : DayOfAdventTests<Day25>
{
  public Day25Tests(ITestOutputHelper output) : base(output) { }

  [Theory]
  [InlineData(1, 2, 18749137)]
  [InlineData(2, 1, 31916031)]
  [InlineData(3, 2, 8057251)]
  [InlineData(4, 5, 10600672)]
  public void Part1(int row, int col, long expected) {
    day.Code = new(row, col);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }
}