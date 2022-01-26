using System.Text.Json;

namespace Advent2015;

public class Day12 : DayOfAdvent<Day12>, IDayOfAdvent {
  public int Part1() =>
    _input
      .Split('[', ']', '{', '}', ',', ':')
      .Select(x => int.TryParse(x, out var v) ? v : 0)
      .Sum();
  public string Part1Result() =>
    $"{Part1()}";

  bool JsonRedValued(JsonProperty p) =>
    p.Value.ValueKind == JsonValueKind.String && p.Value.GetString() == "red";

  public int JsonSum(JsonElement json) =>
    json.ValueKind switch {
      JsonValueKind.Object =>
        json.EnumerateObject().Any(JsonRedValued) ? 0
          : json.EnumerateObject().Sum(p => JsonSum(p.Value)),
      JsonValueKind.Array =>
        json.EnumerateArray().Sum(JsonSum),
      JsonValueKind.Number =>
        json.GetInt32(),
      _ => 0,
    };

  public int Part2() =>
    JsonSum(JsonDocument.Parse(_input).RootElement);

  public string Part2Result() =>
    $"{Part2()}";
}
