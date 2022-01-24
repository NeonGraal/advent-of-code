using Xunit;

namespace Advent2015.Test;
public class Day8Tests {
  readonly Day8 day = new();

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1();

    Assert.Equal(12, result);
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");

    var result = day.Part2();

    Assert.Equal(19, result);
  }
}