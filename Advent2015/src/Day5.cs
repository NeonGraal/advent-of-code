namespace Advent2015;

public class Day5 : DayOfAdvent<Day5>, IDayOfAdvent {
  public int Part1() =>
    Lines().Count(IsNice1);
  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() =>
    Lines().Count(IsNice2);
  // 392 is too high
  // Not 77 or 37 or 52

  public string Part2Result() =>
    $"{Part2()}";

  static bool IsVowel(char letter) =>
    letter is 'a' or 'e' or 'i' or 'o' or 'u';

  public static bool IsNice1(string input) {
    var prev = input[0];
    var vowels = IsVowel(prev) ? 1 : 0;
    var doubled = false;

    foreach (var c in input[1..]) {
      if (c == prev) {
        doubled = true;
      }

      if (IsVowel(c)) {
        vowels++;
      }

      if ((prev == 'a' && c == 'b') ||
          (prev == 'c' && c == 'd') ||
          (prev == 'p' && c == 'q') ||
          (prev == 'x' && c == 'y')) {
        return false;
      }

      prev = c;
    }

    return doubled && vowels > 2;
  }

  public static bool IsNice2(string input) {
    var pair = (input[0], input[1]);
    var pairs = new HashSet<(char, char)>();
    var prev = pair;

    var skipPair = false;
    var pairDouble = false;

    foreach (var c in input[2..]) {
      pair = (prev.Item2, c);
      if (pairs.Contains(pair)) {
        pairDouble = true;
      }

      pairs.Add(prev);
      if (prev.Item1 == c) {
        skipPair = true;
      }

      if (pairDouble && skipPair) {
        return true;
      }

      prev = pair;
    }
    return false;
  }
}
