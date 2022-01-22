using Xunit;

namespace $rootnamespace$;

public class $safeitemname$Tests
{
  readonly $safeitemname$ day = new();

  [Theory]
  [InlineData("", 0)]
  public void Part1(string input, int expected)
  {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("", 1)]
  public void Part2(string input, int expected)
  {
    day.SetInput(input);

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}