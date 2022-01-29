namespace Advent2015;

public class MapInt : Map<int>, IMapInt
{
  public int Total { get; set; }
  public MapInt() => Total = 0;
  public MapInt(IMapInt mapInt) : base(mapInt) =>
    Total = mapInt.Total;
}
