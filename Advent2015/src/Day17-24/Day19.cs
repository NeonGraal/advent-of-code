namespace Advent2015;

public class Day19 : DayOfAdvent<Day19>, IDayOfAdvent
{
  public string Medicine = "";

  static ILookup<string, string> Replacements = Array.Empty<string>().ToLookup(s => s);
  static HashSet<int> KeySizes = new HashSet<int>();

  record struct Mol(int Steps, string Molecule)
  {
    public int Len = Molecule.Length;

    string[] ReplaceAt(int start, int end) {
      var local = this;
      return Replacements[Molecule[start..end]]
        .Select(r => local.Molecule[..start] + r + local.Molecule[end..])
        .ToArray();
    }
    public string[] ReplaceAll() {
      var local = this;
      return KeySizes
        .Where(s => local.Len >= s)
        .SelectMany(s =>
          Enumerable.Range(0, local.Len - s + 1)
          .SelectMany(i => local.ReplaceAt(i, i + s))
        ).Distinct().ToArray();
    }
  }

  public int Part1() {
    var lines = Lines();
    if (string.IsNullOrEmpty(Medicine)) {
      Medicine = lines[^1];
    }
    Replacements = lines[..^2].Select(l => l.Split(" => ")).ToLookup(r => r[0], r => r[1]);
    KeySizes = Replacements.Select(l => l.Key.Length).ToHashSet();

    var mol = new Mol(1, Medicine);

    return mol.ReplaceAll().Length;
  }

  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() {
    var lines = Lines();
    if (string.IsNullOrEmpty(Medicine)) {
      Medicine = lines[^1];
    }
    Replacements = lines[..^2].Select(l => l.Split(" => ")).ToLookup(r => r[1], r => r[0]);
    KeySizes = Replacements.Select(l => l.Key.Length).ToHashSet();

    var steps = 0;
    var count = steps;
    var possibles = new List<Mol> { new Mol(steps, Medicine) };
    var tried = new HashSet<string>();
    var curr = possibles.MinBy(m => (m.Len, m.Steps));

    while (curr.Molecule != "e") {
      if (++count % 10000 == 0) {
        Console.WriteLine($"Progress {count} - {possibles.Count}");
      }

      possibles.Remove(curr);
      tried.Add(curr.Molecule);
      steps = curr.Steps + 1;
      possibles.AddRange(curr
        .ReplaceAll()
        .Except(tried)
        .Select(m => new Mol(steps, m))
      );
      curr = possibles.MinBy(m => (m.Len, m.Steps));
    }

    return curr.Steps;
  }
  public string Part2Result() =>
    $"{Part2()}";
}