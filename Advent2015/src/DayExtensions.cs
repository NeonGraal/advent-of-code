namespace Advent2015;

public static class DayExtensions
{
  public static string Show<T>(this IEnumerable<T> list) =>
    Show(list, " ", format: t => t?.ToString() ?? "");
  public static string Show<T>(this IEnumerable<T> list, Func<T, string> format) =>
    Show(list, " ", format);
  public static string Show<T>(this IEnumerable<T> list, string sep) =>
    Show(list, sep, format: t => t?.ToString() ?? "");
  public static string Show<T>(this IEnumerable<T> list, string sep, Func<T, string> format) =>
    string.Join(sep, list.Select(format));
}

public interface IMap<T> : IDictionary<string, T> { }

public class Map<T> : Dictionary<string, T>, IMap<T>
{
  public Map() { }
  public Map(IMap<T> dictionary) : base(dictionary) { }
}