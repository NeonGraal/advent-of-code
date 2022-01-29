namespace Advent2015;

public class Day16 : DayOfAdvent<Day16>, IDayOfAdvent
{
  readonly MapInt Readings1 = new() {
    ["children"] = 3,
    ["cats"] = 7,
    ["samoyeds"] = 2,
    ["pomeranians"] = 3,
    ["akitas"] = 0,
    ["vizslas"] = 0,
    ["goldfish"] = 5,
    ["trees"] = 3,
    ["cars"] = 2,
    ["perfumes"] = 1,
  };

  static (int Id, MapInt Map) Parse(string line) {
    var parts = line.Split(new char[] { ' ', ':', ',' }, StringSplitOptions.RemoveEmptyEntries);
    var nums = parts.ToInts(-1);

    var map = new MapInt();
    for (var i = 2; i < nums.Length; i += 2) {
      map[parts[i]] = nums[i + 1];
    }

    return (nums[1], map);
  }

  public int Part1() =>
    Lines().Select(Parse)
    .FirstOrDefault(s =>
      s.Map.All(k =>
        Readings1[k.Key] == k.Value)
    ).Id;
  public string Part1Result() =>
    $"{Part1()}";

  readonly Map<Func<int, bool>> Readings2 = new() {
    ["children"] = i => i == 3,
    ["cats"] = i => i > 7,
    ["samoyeds"] = i => i == 2,
    ["pomeranians"] = i => i < 3,
    ["akitas"] = i => i == 0,
    ["vizslas"] = i => i == 0,
    ["goldfish"] = i => i < 5,
    ["trees"] = i => i > 3,
    ["cars"] = i => i == 2,
    ["perfumes"] = i => i == 1,
  };
  public int Part2() =>
    Lines().Select(Parse)
    .FirstOrDefault(s =>
      s.Map.All(k =>
        Readings2[k.Key](k.Value))
    ).Id;

  public string Part2Result() =>
    $"{Part2()}";
}
