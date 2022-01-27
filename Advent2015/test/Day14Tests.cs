namespace Advent2015.Test;
public class Day14Tests : DayOfAdventTests<Day14>
{
  public Day14Tests(ITestOutputHelper output) : base(output) { }

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