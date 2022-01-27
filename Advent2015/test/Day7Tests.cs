namespace Advent2015.Test;
public class Day7Tests : DayOfAdventTests<Day7>
{
  public Day7Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void Part1() {
    day.SampleInput("");

    var result = day.Part1("y");

    Assert.Equal(72, day.Wire["d"].Invoke());
    Assert.Equal(507, day.Wire["e"].Invoke());
    Assert.Equal(492, day.Wire["f"].Invoke());
    Assert.Equal(114, day.Wire["g"].Invoke());
    Assert.Equal(65412, day.Wire["h"].Invoke());
    Assert.Equal(65079, day.Wire["i"].Invoke());
    Assert.Equal(123, day.Wire["x"].Invoke());
    Assert.Equal(456, day.Wire["y"].Invoke());
  }

  [Fact]
  public void Part2() {
    day.SampleInput("");
    day.Wire["a"] = () => 0;

    var result = day.Part2();

    Assert.Equal(16076, day.Wire["b"].Invoke());
  }
}