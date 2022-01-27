namespace Advent2015.Test;

public class DayOfAdventTests<T>
  where T : IDayOfAdvent, new()
{
  protected readonly T day;

  public DayOfAdventTests(ITestOutputHelper output) =>
    day = new T() { Console = new TestOutput(output) };

  class TestOutput : IOutput
  {
    readonly ITestOutputHelper _output;

    public TestOutput(ITestOutputHelper output) =>
      _output = output;

    public void WriteLine(string message) =>
      _output.WriteLine(message);
  }
}