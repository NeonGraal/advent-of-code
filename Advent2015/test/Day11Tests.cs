namespace Advent2015.Test;
public class Day11Tests {
  readonly Day11 day = new();

  [Theory]
  [InlineData("hijklmmn", true)]
  [InlineData("abbceffg", false)]
  [InlineData("abbcegjk", false)]
  public void Requirement1(string input, bool expected) {
    var result = day.Requirement1(input);

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("hijklmmn", false)]
  [InlineData("abbceffg", true)]
  [InlineData("abbcegjk", true)]
  public void Requirement2(string input, bool expected) {
    var result = day.Requirement2(input);

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("hijklmmn", false)]
  [InlineData("abbceffg", true)]
  [InlineData("abbcegjk", false)]
  [InlineData("aaaabcde", true)]
  public void Requirement3(string input, bool expected) {
    var result = day.Requirement3(input);

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("aaaaaaaa", "aaaaaaab")]
  [InlineData("abcdefgh", "abcdefgj")]
  [InlineData("abcdefgz", "abcdefha")]
  [InlineData("abcdkzzz", "abcdmaaa")]
  [InlineData("nzzzzzzz", "paaaaaaa")]
  [InlineData("zzzzzzzz", "aaaaaaaa")]
  public void Increment(string input, string expected) {
    var result = day.Increment(input);

    Assert.Equal(expected, result);
  }

  [Theory]
  [InlineData("abcdefgh", "abcdffaa")]
  [InlineData("ghijklmn", "ghjaabcc")]
  public void Part1(string input, string expected) {
    day.SetInput(input);

    var result = day.Part1();

    Assert.Equal(expected, result);
  }
}