namespace Advent2015.Test;

public class Day22Tests : DayOfAdventTests<Day22>
{
  public Day22Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void Game1() {
    var game = new Day22.GameState(13, 8, 10, 250);

    game.StartTurn();
    Assert.False(game.GameOver);

    game.Cast(Day22.Spell.Poison);
    Assert.Equal(77, game.Mana);
    Assert.Equal(6, game.Poison);
    Assert.Equal(173, game.Spent);
    Assert.False(game.GameOver);

    game.StartTurn();
    Assert.Equal(5, game.Poison);
    Assert.Equal(10, game.BossHp);
    Assert.False(game.GameOver);

    game.BossHits();
    Assert.Equal(2, game.PlayerHp);
    Assert.False(game.GameOver);

    game.StartTurn();
    Assert.Equal(4, game.Poison);
    Assert.Equal(7, game.BossHp);
    Assert.False(game.GameOver);

    game.Cast(Day22.Spell.MagicMissile);
    Assert.Equal(24, game.Mana);
    Assert.Equal(3, game.BossHp);
    Assert.Equal(226, game.Spent);
    Assert.False(game.GameOver);

    game.StartTurn();
    Assert.Equal(3, game.Poison);
    Assert.Equal(0, game.BossHp);

    Assert.True(game.GameOver);
    Assert.True(game.PlayerWins);
  }

  [Fact]
  public void Game2() {
    var game = new Day22.GameState(14, 8, 10, 250);

    game.StartTurn();
    Assert.False(game.GameOver);

    game.Cast(Day22.Spell.Recharge);
    Assert.Equal(21, game.Mana);
    Assert.Equal(5, game.Recharge);
    Assert.Equal(229, game.Spent);
    Assert.False(game.GameOver);

    game.PlayTurns();
    Assert.Equal(2, game.PlayerHp);
    Assert.Equal(3, game.Recharge);
    Assert.Equal(223, game.Mana);
    Assert.False(game.GameOver);

    game.Cast(Day22.Spell.Shield);
    Assert.Equal(110, game.Mana);
    Assert.Equal(6, game.Shield);
    Assert.Equal(342, game.Spent);
    Assert.False(game.GameOver);

    game.PlayTurns();
    Assert.Equal(1, game.PlayerHp);
    Assert.Equal(1, game.Recharge);
    Assert.Equal(312, game.Mana);
    Assert.Equal(4, game.Shield);
    Assert.Equal(7, game.Armor);
    Assert.False(game.GameOver);

    game.Cast(Day22.Spell.Drain);
    Assert.Equal(239, game.Mana);
    Assert.Equal(3, game.PlayerHp);
    Assert.Equal(12, game.BossHp);
    Assert.Equal(415, game.Spent);
    Assert.False(game.GameOver);

    game.PlayTurns();
    Assert.Equal(0, game.Recharge);
    Assert.Equal(340, game.Mana);
    Assert.Equal(2, game.PlayerHp);
    Assert.Equal(2, game.Shield);
    Assert.Equal(7, game.Armor);
    Assert.False(game.GameOver);

    game.Cast(Day22.Spell.Poison);
    Assert.Equal(167, game.Mana);
    Assert.Equal(6, game.Poison);
    Assert.Equal(588, game.Spent);
    Assert.False(game.GameOver);

    game.PlayTurns();
    Assert.Equal(1, game.PlayerHp);
    Assert.Equal(0, game.Shield);
    Assert.Equal(0, game.Armor);
    Assert.Equal(4, game.Poison);
    Assert.Equal(6, game.BossHp);
    Assert.False(game.GameOver);

    game.Cast(Day22.Spell.MagicMissile);
    Assert.Equal(114, game.Mana);
    Assert.Equal(2, game.BossHp);
    Assert.Equal(641, game.Spent);
    Assert.False(game.GameOver);

    game.PlayTurns();
    Assert.Equal(3, game.Poison);
    Assert.Equal(-1, game.BossHp);

    Assert.True(game.GameOver);
    Assert.True(game.PlayerWins);
  }
}