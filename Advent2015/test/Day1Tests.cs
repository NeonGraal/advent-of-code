using Xunit;

namespace Advent2015.Test;

public class Day1Tests
{
  readonly Day1 day1 = new();

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
  public void Part1(string input, int expected)
  {
    day1.SetInput(input);

    var floor = day1.Part1();

    Assert.Equal(expected, floor);
  }

  [Theory]
  [InlineData(")", 1)]
  [InlineData("()())", 5)]
  public void Part2(string input, int expected)
  {
    day1.SetInput(input);

    var position = day1.Part2();

    Assert.Equal(expected, position);
  }
}