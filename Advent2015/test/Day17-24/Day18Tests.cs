namespace Advent2015.Test;

public class Day18Tests : DayOfAdventTests<Day18>
{
  public Day18Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1(4);

    Assert.Equal(4, result);
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");

    var result = day.Part2(5);

    Assert.Equal(17, result);
  }
}