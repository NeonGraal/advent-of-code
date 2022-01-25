using Xunit;

namespace Advent2015.Test;
public class Day10Tests {
  readonly Day10 day = new();

  [Theory]
  [InlineData("1", 2)]
  [InlineData("11", 2)]
  [InlineData("21", 4)]
  [InlineData("1211", 6)]
  [InlineData("111221", 6)]
  public void Part1(string input, int expected) {
    day.SetInput(input);

    var result = day.Part1(1);

    Assert.Equal(expected, result);
  }
}