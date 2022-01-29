namespace Advent2015.Test;

public class Day16Tests : DayOfAdventTests<Day16>
{
  public Day16Tests(ITestOutputHelper output) : base(output) { }

  [Theory]
  [InlineData("Sue 1: goldfish: 9, cars: 0, samoyeds: 9", 0)]
  [InlineData("Sue 2: perfumes: 1, trees: 3, goldfish: 5", 2)]
  public void Part1(string input, int expected) {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("Sue 1: goldfish: 8, cars: 0, samoyeds: 9", 0)]
  [InlineData("Sue 2: perfumes: 1, trees: 4, goldfish: 4", 2)]
  public void Part2(string input, int expected) {
    day.SetInput(input);

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}