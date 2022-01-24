namespace Advent2015;

public class DayOfAdvent<T>
  where T : IDayOfAdvent, new()
{
  protected string _input = "";

  protected string[] Lines() =>
    _input.Split(Environment.NewLine).ToArray();

  public string DayName { get; } = typeof(T).Name;

  public void LoadInput() =>
    _input = File.ReadAllText($@"input/{DayName}.input");

  public void SampleInput(string suffix) =>
    _input = File.ReadAllText($@"input/{DayName}{suffix}.sample");

  public void SetInput(string input) =>
    _input = input;

  internal static void Run()
  {
    var day1 = new T();
    day1.LoadInput();

    Console.Write($"{day1.DayName} ");
    Console.Write($"- Part 1: {day1.Part1Result()} ");
    Console.WriteLine($"- Part 2: {day1.Part2Result()}");
  }
}
