namespace Advent2015.Test;

public class Day19Tests : DayOfAdventTests<Day19>
{
  public Day19Tests(ITestOutputHelper output) : base(output) { }

  [Theory]
  [InlineData("HOH", 4)]
  [InlineData("HOHOHO", 7)]
  public void Part1(string input, int expected) {
    day.SampleInput("-1");
    day.Medicine = input;

    var result = day.Part1();

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("HOH", 3)]
  [InlineData("HOHOHO", 6)]
  public void Part2(string input, int expected) {
    day.SampleInput("-2");
    day.Medicine = input;

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}