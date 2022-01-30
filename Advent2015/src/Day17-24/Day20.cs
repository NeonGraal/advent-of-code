namespace Advent2015;

public class Day20 : DayOfAdvent<Day20>, IDayOfAdvent
{
  public new void LoadInput() { }

  static int[] Factor(int num, int fact) {
    if (num % fact != 0) {
      return Array.Empty<int>();
    }
    var denom = num / fact;
    return denom == fact
      ? new[] { fact }
      : new[] { fact, denom };
  }

  static int Sum10Factors(int num) =>
    Enumerable.Range(1, (int)Math.Sqrt(num))
    .SelectMany(i => Factor(num, i))
    .Sum() * 10;

  public int Part1(int presents) {
    var input = presents / 10;
    var maxHouse = input - 1;

    var minFactor = 1;
    var minHouse = 1;
    var minTotal = 1;
    while (minTotal + minHouse * (minFactor + 1) < input) {
      minHouse *= ++minFactor;
      minTotal += minHouse;
    }

    Console.WriteLine($"{presents}: {minFactor}! {minHouse} to {maxHouse}");

    for (var num = minHouse; num <= maxHouse; num += 2) {
      minTotal = Sum10Factors(num);
      if (minTotal >= presents) {
        return num;
      }
    }

    return -1;
  }
  public string Part1Result() =>
    $"{Part1(29000000)}";

  static int Sum11Factors(int num) =>
    Enumerable.Range(1, (int)Math.Sqrt(num))
    .Select(i => Factor(num, i))
    .Where(f => f.Any(s => s <= 50))
    .SelectMany(f => f).Sum() * 11;

  public int Part2(int presents) {
    var input = presents / 11;
    var maxHouse = input - 1;

    var minFactor = 1;
    var minHouse = 1;
    var minTotal = 1;
    while (minTotal + minHouse * (minFactor + 1) < input) {
      minHouse *= ++minFactor;
      minTotal += minHouse;
    }

    Console.WriteLine($"{presents}: {minFactor}! {minHouse} to {maxHouse}");

    for (var num = minHouse; num <= maxHouse; num += 2) {
      minTotal = Sum11Factors(num);
      if (minTotal >= presents) {
        return num;
      }
    }

    return -1;
  }
  public string Part2Result() =>
    $"{Part2(29000000)}";
}