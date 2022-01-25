namespace Advent2015;

public class Day10 : DayOfAdvent<Day10>, IDayOfAdvent {
  public int Part1(int rounds) {
    var seq = _input.ToCharArray();
    var curr = ' ';
    var count = '0';
    var result = new List<char>();

    for (var i = 0; i < rounds; i++) {
      foreach (var c in seq) {
        if (c == curr) {
          count++;
        } else {
          if (curr > ' ') {
            result.Add(count);
            result.Add(curr);
          }
          curr = c;
          count = '1';
        }
      }
      if (count > '0') {
        result.Add(count);
        result.Add(curr);
      }
      seq = result.ToArray();
      curr = ' ';
      count = '0';
      result.Clear();
    }

    return seq.Length;
  }

  // 708004 is too high
  // 527432 is too high

  public string Part1Result() =>
    $"{Part1(40)}";

  public string Part2Result() =>
    $"{Part1(50)}";
}
