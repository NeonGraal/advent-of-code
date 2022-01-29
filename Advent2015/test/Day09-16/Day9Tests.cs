namespace Advent2015.Test;
public class Day9Tests : DayOfAdventTests<Day9>
{
  public Day9Tests(ITestOutputHelper output) : base(output) { }

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