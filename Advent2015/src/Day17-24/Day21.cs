namespace Advent2015;

public class Day21 : DayOfAdvent<Day21>, IDayOfAdvent
{
  public new void LoadInput() => Boss = new(103, 9, 2, 0);

  public record struct Player(int HP, int Dmg, int Arm, int Cost)
  {
    public static Player WithItems(int hp, params Item[] items) =>
      new(hp, items.Sum(i => i.Dmg), items.Sum(i => i.Arm), items.Sum(i => i.Cost));
    public void HitBy(Player attacker) =>
      HP -= Math.Max(1, attacker.Dmg - Arm);
    public bool Alive => HP > 0;
  }

  public static bool PlayerWinsFight(Player player, Player enemy) {
    while (player.Alive) {
      enemy.HitBy(player);
      if (!enemy.Alive) {
        return true;
      }
      player.HitBy(enemy);
    }
    return false;
  }

  public record struct Item(int Cost, int Dmg, int Arm);

  readonly Item[] Weapons = { new(8, 4, 0), new(10, 5, 0), new(25, 6, 0), new(40, 7, 0), new(74, 8, 0) };
  readonly Item[] Armour = { new(13, 0, 1), new(31, 0, 2), new(53, 0, 2), new(75, 0, 4), new(102, 0, 5) };
  readonly Item[] Rings = { new(25, 1, 0), new(50, 2, 0), new(100, 3, 0), new(20, 0, 1), new(40, 0, 2), new(80, 0, 3) };

  public Player Boss { get; private set; }

  IEnumerable<Player> PossiblePlayers() =>
    Weapons.SelectMany(w =>
      Armour.SelectMany(a =>
        Rings.SelectMany(r1 =>
          Rings.Where(r2 => r2 != r1)
          .Select(r2 => Player.WithItems(100, w, a, r1, r2))
          .Append(Player.WithItems(100, w, a, r1))
        ).Append(Player.WithItems(100, w, a))
      ).Append(Player.WithItems(100, w)));

  public int Part1() =>
    PossiblePlayers()
    .Where(p => PlayerWinsFight(p, Boss))
    .Select(p => p.Cost)
    .Min();
  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() =>
    PossiblePlayers()
    .Where(p => !PlayerWinsFight(p, Boss))
    .Select(p => p.Cost)
    .Max();
  public string Part2Result() =>
    $"{Part2()}";
}