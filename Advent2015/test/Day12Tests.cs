using Xunit;

namespace Advent2015.Test;
public class Day12Tests {
  readonly Day12 day = new();

  [Theory]
  [InlineData("[1,2,3]", 6)]
  [InlineData("{\"a\":2,\"b\":4}", 6)]
  [InlineData("[[[3]]]", 3)]
  [InlineData("{\"a\":{\"b\":4},\"c\":-1}", 3)]
  [InlineData("{\"a\":[-1,1]}]", 0)]
  [InlineData("[-1,{\"a\":1}]", 0)]
  [InlineData("[]", 0)]
  [InlineData("{}", 0)]
  public void Part1(string input, int expected) {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("[1,2,3]", 6)]
  [InlineData("[1,{\"c\":\"red\",\"b\":2},3]", 4)]
  [InlineData("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", 0)]
  [InlineData("[1,\"red\",5]", 6)]
  public void Part2(string input, int expected) {
    day.SetInput(input);

    var result = day.Part2();

    Assert.Equal(expected, result);
  }
}