global using Xunit;
global using Xunit.Abstractions;

namespace Advent2015.Test;

public class Day1Tests : DayOfAdventTests<Day1>
{
  public Day1Tests(ITestOutputHelper output) : base(output) { }

  [Theory]
  [InlineData("(())", 0)]
  [InlineData("()()", 0)]
  [InlineData("(((", 3)]
  [InlineData("(()(()(", 3)]
  [InlineData("))(((((", 3)]
  [InlineData("())", -1)]
  [InlineData("))(", -1)]
  [InlineData(")))", -3)]
  [InlineData(")())())", -3)]
  public void Part1(string input, int expected) {
    day.SetInput(input);

    var floor = day.Part1();

    Assert.Equal(expected, floor);
  }

  [Theory]
  [InlineData(")", 1)]
  [InlineData("()())", 5)]
  public void Part2(string input, int expected) {
    day.SetInput(input);

    var position = day.Part2();

    Assert.Equal(expected, position);
  }
}