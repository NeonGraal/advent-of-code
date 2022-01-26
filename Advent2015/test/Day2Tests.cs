namespace Advent2015.Test;
public class Day2Tests {
  readonly Day2 day = new();

  [Theory]
  [InlineData("2x3x4", 58)]
  [InlineData("1x1x10", 43)]
  public void Part1(string input, int expected) {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("2x3x4", 34)]
  [InlineData("1x1x10", 14)]
  public void Part2(string input, int expected) {
    day.SetInput(input);

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}