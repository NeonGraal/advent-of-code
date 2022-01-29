namespace Advent2015.Test;

public class Day17Tests : DayOfAdventTests<Day17>
{
  public Day17Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1(25);

    Assert.Equal(4, result);
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");

    var result = day.Part2(25);

    Assert.Equal(3, result);
  }
}