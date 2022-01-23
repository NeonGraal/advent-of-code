using Xunit;

namespace Advent2015.Test;
public class Day6Tests
{
  readonly Day6 day = new();

  [Theory]
  [InlineData("turn on 0,0 through 999,999", 1000000)]
  [InlineData("turn on 300,300 through 499,499", 70100)]
  [InlineData("toggle 0,0 through 999,0", 50900)]
  [InlineData("turn off 499,499 through 500,500", 50098)]
  public void Part1(string input, int expected)
  {
    day.SetInput(input);

    day.Apply1(new Day6.Act(Day6.Action.On, new Day6.Rect(400, 0, 499, 500))); // 50100

    var result = day.Part1();

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("turn on 0,0 through 0,0", 1)]
  [InlineData("toggle 0,0 through 999,999", 2000000)]
  public void Part2(string input, int expected)
  {
    day.SetInput(input);

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}