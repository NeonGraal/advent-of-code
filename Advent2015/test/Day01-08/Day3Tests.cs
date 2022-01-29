namespace Advent2015.Test;
public class Day3Tests : DayOfAdventTests<Day3>
{
  public Day3Tests(ITestOutputHelper output) : base(output) { }

  [Theory]
  [InlineData(">", 2)]
  [InlineData("^>v<", 4)]
  [InlineData("^v^v^v^v^v", 2)]
  public void Part1(string input, int expected) {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("^v", 3)]
  [InlineData("^>v<", 3)]
  [InlineData("^v^v^v^v^v", 11)]
  public void Part2(string input, int expected) {
    day.SetInput(input);

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}