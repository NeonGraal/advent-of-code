namespace Advent2015.Test;
public class Day4Tests : DayOfAdventTests<Day4>
{
  public Day4Tests(ITestOutputHelper output) : base(output) { }

  [Theory]
  [InlineData("abcdef", 609043)]
  [InlineData("pqrstuv", 1048970)]
  public void Part1(string input, int expected) {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }
}