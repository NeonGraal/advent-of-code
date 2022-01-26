namespace Advent2015.Test;
public class Day14Tests {
  readonly Day14 day = new();

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1(1000);

    Assert.Equal(1120, result);
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");

    var result = day.Part2(1000);

    Assert.Equal(689, result);
  }
}