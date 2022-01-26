namespace Advent2015.Test;
public class Day5Tests {
  [Theory]
  [InlineData("ugknbfddgicrmopn", true)]
  [InlineData("aaa", true)]
  [InlineData("jchzalrnumimnmhp", false)]
  [InlineData("haegwjzuvuyypxyu", false)]
  [InlineData("dvszwmarrgswjxmb", false)]
  public void IsNice1(string input, bool expected) {
    var result = Day5.IsNice1(input);

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("qjhvhtzxzqqjkmpb", true)]
  [InlineData("xxyxx", true)]
  [InlineData("aaa", false)]
  [InlineData("uurcxstgmygtbstg", false)]
  [InlineData("ieodomkazucvgmuy", false)]
  public void IsNice2(string input, bool expected) {
    var result = Day5.IsNice2(input);

    Assert.Equal(expected, result);
  }
}