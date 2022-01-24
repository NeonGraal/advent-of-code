using Xunit;

namespace Advent2015.Test;
public class Day9Tests {
  readonly Day9 day = new();

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1();

    Assert.Equal(605, result);
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");

    var result = day.Part2();

    Assert.Equal(982, result);
  }
}