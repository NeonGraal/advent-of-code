namespace Advent2015;

public class Day11 : DayOfAdvent<Day11>, IDayOfAdvent {
  public string Part1() {
    var result = Increment(_input);
    while (!(Requirement1(result) && Requirement2(result) && Requirement3(result))) {
      result = Increment(result);
    }

    return result;
  }
  public string Part1Result() =>
    $"{Part1()}";

  public string Part2() {
    _input = Part1();
    return Part1();
  }
  public string Part2Result() =>
    $"{Part2()}";

  public bool Requirement1(string input) {
    var first = input[0];
    var second = input[1];
    foreach (var c in input[2..]) {
      if (first + 1 == second && second + 1 == c) {
        return true;
      }

      (first, second) = (second, c);
    }
    return false;
  }

  public bool Requirement2(string input) =>
    input.All(c => c is not 'i' or 'l' or 'o');

  public bool Requirement3(string input) {
    var prev = input[0];
    var count = 0;
    foreach (var c in input[1..]) {
      if (prev == c) {
        if (++count > 1) {
          return true;
        }

        prev = ' ';
      } else {
        prev = c;
      }
    }
    return false;
  }

  public string Increment(string input) {
    var rest = input[0..^1];
    var next = input[^1];
    if (next == 'z') {
      return rest.Length == 0 ? "a"
        : Increment(rest) + 'a';
    }
    next++;
    if (next is 'i' or 'l' or 'o') {
      next++;
    }

    return rest + next;
  }
}
