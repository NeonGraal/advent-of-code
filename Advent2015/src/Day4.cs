using System.Security.Cryptography;
using System.Text;

namespace Advent2015;

public class Day4 : DayOfAdvent<Day4>, IDayOfAdvent
{
  int FindStarting(string starts)
  {
    var md5 = MD5.Create();
    for (var i = 1; i < int.MaxValue; i++)
    {
      var hash = md5.ComputeHash(Encoding.ASCII.GetBytes(_input + i.ToString()));
      var hex = Convert.ToHexString(hash);
      if (hex.StartsWith(starts)) return i;
    }
    return -1;
  }

  public int Part1() =>
    FindStarting("00000");
  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() =>
    FindStarting("000000");
  public string Part2Result() =>
    $"{Part2()}";
}
