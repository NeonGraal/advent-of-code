namespace Advent2015.Test;
public class Day13Tests : DayOfAdventTests<Day13>
{
  public Day13Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1();

    Assert.Equal(330, result);
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");

    var result = day.Part2();

    Assert.Equal(286, result);
  }
}