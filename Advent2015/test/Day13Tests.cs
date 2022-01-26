using Xunit;

namespace Advent2015.Test;
public class Day13Tests {
  readonly Day13 day = new();

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1();

    Assert.Equal(330, result);
  }

  [Theory]
  [InlineData("", 1)]
  public void Part2(string input, int expected) {
    day.SetInput(input);

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}