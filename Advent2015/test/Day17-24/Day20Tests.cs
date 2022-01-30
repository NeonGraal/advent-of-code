namespace Advent2015.Test;

public class Day20Tests : DayOfAdventTests<Day20>
{
  public Day20Tests(ITestOutputHelper output) : base(output) { }

  [Theory]
  [InlineData(90, 6)]
  [InlineData(280, 12)]
  [InlineData(630, 30)]
  [InlineData(910, 36)]
  public void Part1(int input, int expected) {
    var result = day.Part1(input);

    Assert.Equal(expected, result);
  }
}