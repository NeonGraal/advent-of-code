namespace Advent2015.Test;
public class Day4Tests {
  readonly Day4 day = new();

  [Theory]
  [InlineData("abcdef", 609043)]
  [InlineData("pqrstuv", 1048970)]
  public void Part1(string input, int expected) {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }
}