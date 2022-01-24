namespace Advent2015;

public class Day8 : DayOfAdvent<Day8>, IDayOfAdvent {
  int DecodedLength(string line) {
    var len = 0;
    for (var i = 1; i < line.Length - 1; i++) {
      if (line[i] == '\\') {
        switch (line[i + 1]) {
          case 'x':
            i += 3;
            break;
          case '\\':
          case '"':
            i++;
            break;
        }
      }
      len++;
    }
    return len;
  }

  int EncodedLength(string line)
    => 2 + line.Length + line.Count(c => c is '"' or '\\');

  public int Part1() =>
    Lines().Select(l => l.Length - DecodedLength(l)).Sum();
  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() =>
    Lines().Select(l => EncodedLength(l) - l.Length).Sum();

  public string Part2Result() =>
    $"{Part2()}";
}
