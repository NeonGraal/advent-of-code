namespace Advent2015.Test;

public class Day21Tests : DayOfAdventTests<Day21>
{
  public Day21Tests(ITestOutputHelper output) : base(output) { }

  [Fact]
  public void PlayerJustWins() {
    var player = new Day21.Player(8, 5, 5, 0);
    var boss = new Day21.Player(12, 7, 2, 0);

    var playerWins = Day21.PlayerWinsFight(player, boss);

    Assert.True(playerWins);
  }
  [Fact]
  public void PlayerLoses() {
    var player = new Day21.Player(8, 5, 5, 0);
    var boss = new Day21.Player(12, 7, 3, 0);

    var playerWins = Day21.PlayerWinsFight(player, boss);

    Assert.True(!playerWins);
  }
}