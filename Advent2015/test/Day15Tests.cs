using System.Linq;

namespace Advent2015.Test;
public class Day15Tests : DayOfAdventTests<Day15>
{
  public Day15Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1();

    Assert.Equal(62842880, result);
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");

    var result = day.Part2();

    Assert.Equal(0, result);
  }
}