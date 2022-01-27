namespace Advent2015.Test;
public class Day8Tests : DayOfAdventTests<Day8>
{
  public Day8Tests(ITestOutputHelper output) : base(output) { }

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