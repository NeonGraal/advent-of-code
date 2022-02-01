namespace Advent2015.Test;

public class Day23Tests : DayOfAdventTests<Day23>
{
  public Day23Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1("a");

    Assert.Equal(2, result);
  }

  [Fact]
  public void Part2() {
    var expected = 0;
    day.SampleInput("");

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}