namespace Advent2015;

public class Map<T> : Dictionary<string, T>, IMap<T>
{
  public Map() { }
  public Map(IDictionary<string, T> dictionary) : base(dictionary) { }
}