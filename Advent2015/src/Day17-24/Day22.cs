namespace Advent2015;

public class Day22 : DayOfAdvent<Day22>, IDayOfAdvent
{
  public new void LoadInput() => Start = new(71, 10);

  public enum Spell
  {
    MagicMissile = 53, Drain = 73, Shield = 113, Poison = 173, Recharge = 229
  }

  public class GameState
  {
    public GameState(int bossHp, int bossDmg) {
      BossHp = bossHp;
      BossDmg = bossDmg;
      PlayerHp = 50;
      Mana = 500;
    }
    public GameState(int bossHp, int bossDmg, int playerHp, int mana) {
      BossHp = bossHp;
      BossDmg = bossDmg;
      PlayerHp = playerHp;
      Mana = mana;
    }

    static readonly Dictionary<Spell, Action<GameState>> _actions = new() {
      [Spell.MagicMissile] = g => g.BossHp -= 4,
      [Spell.Drain] = g => { g.BossHp -= 2; g.PlayerHp += 2; },
      [Spell.Shield] = g => g.Shield = 6,
      [Spell.Poison] = g => g.Poison = 6,
      [Spell.Recharge] = g => g.Recharge = 5,
    };

    public int BossHp { get; private set; }
    public int BossDmg { get; private set; }

    public void BossHits() =>
      PlayerHp -= BossDmg > Armor ? BossDmg - Armor : 1;

    public int PlayerHp { get; private set; }

    public bool GameOver => BossHp <= 0 || PlayerHp <= 0;
    public bool PlayerWins => BossHp <= 0 && PlayerHp > 0;

    public int Shield { get; private set; }
    public int Armor => Shield > 0 ? 7 : 0;
    public int Poison { get; private set; }
    public int Recharge { get; private set; }
    public int Spent { get; private set; }
    public int Mana { get; private set; }

    public override string ToString() =>
      $"Boss{BossDmg}: {BossHp} Player: {PlayerHp} {Mana} ({Shield}/{Poison}/{Recharge}) {Spent}";

    public void StartTurn() {
      if (Shield > 0) { Shield--; }
      if (Poison > 0) { BossHp -= 3; Poison--; }
      if (Recharge > 0) { Mana += 101; Recharge--; }
    }
    public void Cast(Spell spell) {
      Spent += (int)spell;
      Mana -= (int)spell;
      _actions[spell](this);
    }

    public void PlayTurns(int dmg) {
      PlayerHp -= dmg;
      if (!GameOver) {
        StartTurn();
        if (!GameOver) {
          BossHits();
          if (!GameOver) {
            StartTurn();
          }
        }
      }
    }

    public GameState CloneCastAndPlay(Spell spell, int dmg) {
      var next = (GameState)MemberwiseClone();
      next.Cast(spell);
      if (!next.GameOver) {
        next.PlayTurns(dmg);
      }
      return next;
    }

    public IEnumerable<GameState> PossibleCasts(int dmg) {
      if (Mana > (int)Spell.MagicMissile) {
        yield return CloneCastAndPlay(Spell.MagicMissile, dmg);
      }
      if (Mana > (int)Spell.Drain) {
        yield return CloneCastAndPlay(Spell.Drain, dmg);
      }
      if (Shield < 1 && Mana > (int)Spell.Shield) {
        yield return CloneCastAndPlay(Spell.Shield, dmg);
      }
      if (Poison < 1 && Mana > (int)Spell.Poison) {
        yield return CloneCastAndPlay(Spell.Poison, dmg);
      }
      if (Recharge < 1 && Mana > (int)Spell.Recharge) {
        yield return CloneCastAndPlay(Spell.Recharge, dmg);
      }
    }
  }

  GameState Start = new(0, 0, 0, 0);

  public int Part1() {
    var possibles = new List<GameState> { Start };
    var curr = Start;
    var count = 0;

#pragma warning disable CS8602 // Dereference of a possibly null reference.
    while (!curr.GameOver) {
#pragma warning restore CS8602 // Dereference of a possibly null reference.
      if (++count % 1000 == 0) {
        Console.WriteLine($"Progress {count} - {possibles.Count}");
        Console.WriteLine($"- {curr}");
      }

      possibles.Remove(curr);

      foreach (var s in curr.PossibleCasts(0)) {
        if (s.PlayerWins || !s.GameOver) {
          possibles.Add(s);
        }
      }

      curr = possibles.MinBy(s => (s.BossHp, s.Spent));
    }

    return curr.Spent;
  }
  public string Part1Result() =>
    $"{Part1()}";

  public int Part2() {
    var possibles = new List<GameState> { Start };
    var curr = Start;
    var count = 0;

#pragma warning disable CS8602 // Dereference of a possibly null reference.
    while (!curr.GameOver) {
#pragma warning restore CS8602 // Dereference of a possibly null reference.
      if (++count % 1000 == 0) {
        Console.WriteLine($"Progress {count} - {possibles.Count}");
        Console.WriteLine($"- {curr}");
      }

      possibles.Remove(curr);

      foreach (var s in curr.PossibleCasts(1)) {
        if (s.PlayerWins || !s.GameOver) {
          possibles.Add(s);
        }
      }

      curr = possibles.MinBy(s => (s.BossHp, s.Spent));
    }

    return curr.Spent;
  }
  public string Part2Result() =>
    $"{Part2()}";
}