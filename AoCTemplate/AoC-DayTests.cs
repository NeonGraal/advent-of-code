namespace $safeprojectname$.Test;

public class $safeitemname$Tests : DayOfAdventTests<$safeitemname$>
{
  public $safeitemname$Tests(ITestOutputHelper output) : base(output) { }

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
    day.SampleInput("");

    var result = day.Part2();

    Assert.Equal(0, result);
  }
}