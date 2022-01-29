namespace Advent2015;

public interface IDayOfAdvent
{
  IOutput Console { get; set; }
  string DayName { get; }
  void LoadInput();
  string Part1Result();
  string Part2Result();
}

public class DayOfAdvent<T>
  where T : IDayOfAdvent, new()
{

  public DayOfAdvent() =>
    Console = new ConsoleOutput();

  public IOutput Console { get; set; }

  protected string _input = "";

  protected string[] Lines() =>
    _input.Split(Environment.NewLine);

  protected P[] Lines<P>(Func<string, P> parser) =>
    _input.Split(Environment.NewLine).Select(parser).ToArray();

  public string DayName { get; } = typeof(T).Name;

  public void LoadInput() =>
    _input = File.ReadAllText($@"input/{DayName}.input");

  public void SampleInput(string suffix) =>
    _input = File.ReadAllText($@"input/{DayName}{suffix}.sample");

  public void SetInput(string input) =>
    _input = input;

  internal static void Run() {
    var day = new T();
    day.LoadInput();

    System.Console.WriteLine($"{day.DayName}");
    System.Console.WriteLine($"- Part 1: {day.Part1Result()}");
    System.Console.WriteLine($"- Part 2: {day.Part2Result()}");
  }

  class ConsoleOutput : IOutput
  {
    public void WriteLine(string message) =>
      System.Console.WriteLine(message);
  }
}
