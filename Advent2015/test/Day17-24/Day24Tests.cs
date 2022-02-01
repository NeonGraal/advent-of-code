namespace Advent2015.Test;

public class Day24Tests : DayOfAdventTests<Day24>
{
  public Day24Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1();

    Assert.Equal(99, result);
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");

    var result = day.Part2();

    Assert.Equal(44, result);
  }
}