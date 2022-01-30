namespace $rootnamespace$.Test;

public class $safeitemname$ : DayOfAdventTests<$fileinputname$>
{
  public $safeitemname$(ITestOutputHelper output) : base(output) { }

  [Theory]
  [InlineData("", 0)]
  public void Part1(string input, int expected)
  {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }

  [Fact]
  public void Part2()
  {
    var expected = 0;
    day.SampleInput("");

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}