namespace Advent2015;

public interface IMapInt : IMap<int>
{
  int Total { get; set; }
}

public class MapInt : Map<int>, IMapInt
{
  public int Total { get; set; }
  public MapInt() => Total = 0;
  public MapInt(IMapInt mapInt) : base(mapInt) =>
    Total = mapInt.Total;
}

public class Day15 : DayOfAdvent<Day15>, IDayOfAdvent
{
  public record Ingredient(string Name, MapInt Properties)
  {
    public int this[string k] => Properties[k];
    public int Max { get => _max; set => _max = value < _max ? value : _max; }

    int _max = 100;

    public override string ToString() =>
      $"{Name} (1..{Max}): " + Properties.Show(" ", p => $"{p.Key} * {p.Value}");

    public MapInt[] Possibles(MapInt[] bases, IOutput output) =>
      bases.SelectMany(d => d.Total >= Max
          ? Array.Empty<MapInt>()
          : Enumerable.Range(1, Max)
            .Select(v => new MapInt(d) { [Name] = v, Total = d.Total + v })
            .Where(v => v.Total <= 100)
        ).ShowProgress(1000000, output).ToArray();

    public static Ingredient Parse(string line) {
      var parts = line.Split(new[] { ' ', ',', ':' }, StringSplitOptions.RemoveEmptyEntries);
      var nums = parts.Select(p => int.TryParse(p, out var num) ? num : 0).ToArray();
      var properties = new MapInt {
        ["Cap"] = nums[2],
        ["Dur"] = nums[4],
        ["Fal"] = nums[6],
        ["Tex"] = nums[8],
        ["Cal"] = nums[10],
      };

      return new Ingredient(parts[0], properties);
    }
  }

  readonly string[] properties = new[] { "Cap", "Dur", "Fal", "Tex" };

  void SetMaximums(IEnumerable<Ingredient> ingredients) {
    foreach (var f in properties) {
      var positive = ingredients.Where(i => i[f] > 0).ToArray();
      var negative = ingredients.Where(i => i[f] < 0).ToArray();
      var pos = positive.Sum(i => i[f]);
      var neg = negative.Sum(i => -i[f]);

      foreach (var p in positive) {
        p.Max = 100 * pos / (pos + neg);
      }
    }
  }

  public IEnumerable<IMapInt> Possibles(IEnumerable<Ingredient> ingredients) {
    var all = ingredients.Aggregate(new[] { new MapInt() }, (a, i) => i.Possibles(a, Console));
    return all.Where(v => v.Total == 100).ToArray();
  }

  public int Cookie(IMapInt possible, IMap<Ingredient[]> ingredients) {
    var props = new MapInt();
    var value = 0;

    foreach (var p in properties) {
      value = ingredients[p].Sum(i => possible[i.Name] * i[p]);
      if (value <= 0) {
        return 0;
      }

      props[p] = value;
    }

    return props.Values.Aggregate(1, (t, v) => t * v);
  }

  public int Part1() {
    var ingredients = Lines(Ingredient.Parse);
    SetMaximums(ingredients);
    Console.WriteLine("");

    var propIngreds = new Map<Ingredient[]>();

    foreach (var f in properties) {
      var positive = ingredients.Where(i => i[f] > 0).ToArray();
      var negative = ingredients.Where(i => i[f] < 0).ToArray();
      propIngreds[f] = positive.Concat(negative).ToArray();
      Console.WriteLine($"{f} : {PropertyIngredients(positive, f, 1)} > {PropertyIngredients(negative, f, -1)}");
    }
    Console.WriteLine("");
    Console.WriteLine(ingredients.Show("\n"));

    var cookies = Possibles(ingredients).Select(p => Cookie(p, propIngreds));

    return cookies.Max();

    string PropertyIngredients(Ingredient[] ingredients, string property, int sign) =>
      string.Join(" + ", ingredients.Select(i => $"{sign * i[property]} * {i.Name}"));
  }
  public string Part1Result() =>
    $"{Part1()}";

  public int Calories(IMapInt possible, Ingredient[] ingredients) =>
    ingredients.Sum(i => possible[i.Name] * i["Cal"]);

  public int Part2() {
    var ingredients = Lines(Ingredient.Parse);
    SetMaximums(ingredients);
    var propIngreds = new Map<Ingredient[]>(properties.ToDictionary(p => p,
      f => ingredients.Where(i => i[f] != 0).ToArray()));
    var cookies = Possibles(ingredients).Where(p => Calories(p, ingredients) == 500).Select(p => Cookie(p, propIngreds));
    return cookies.Max();
  }

  public string Part2Result() =>
    $"{Part2()}";
}
