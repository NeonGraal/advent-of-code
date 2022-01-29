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
  public static IEnumerable<T> ShowProgress<T>(this IEnumerable<T> list, int every, IOutput output) =>
    list.Select((t, i) => {
      if (i % every == (every - 1)) {
        output.WriteLine($" {i + 1}");
      }
      return t;
    });

  public static int[] ToInts(this string[] parts, int def) =>
    parts.Select(p => int.TryParse(p, out var v) ? v : def).ToArray();
}
