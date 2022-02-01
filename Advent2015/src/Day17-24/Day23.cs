namespace Advent2015;

public class Day23 : DayOfAdvent<Day23>, IDayOfAdvent
{
  enum Instruct { Hlf, Tpl, Inc, Jmp, Jie, Jio };

  record struct Inst(Instruct ins, string reg, int offset)
  {
    public static Inst Parse(string line) {
      var parts = line.Split(' ', ',');
      var nums = parts.ToInts(0);
      var ins = Enum.Parse<Instruct>(parts[0], true);
      return ins switch {
        Instruct.Hlf or Instruct.Tpl or Instruct.Inc => new Inst(ins, parts[1], 0),
        Instruct.Jmp => new Inst(ins, "", nums[1]),
        Instruct.Jie or Instruct.Jio => new Inst(ins, parts[1], nums[3]),
        _ => throw new NotImplementedException(),
      };
    }
  }

  record State(Inst[] Insts)
  {
    int pc = 0;
    readonly Dictionary<string, int> regs = new() { ["a"] = 0, ["b"] = 0 };
    public int this[string reg] => regs[reg];

    bool Running => pc >= 0 && pc < Insts.Length;

    readonly Dictionary<Instruct, Action<string, int, State>> _actions = new() {
      [Instruct.Hlf] = (r, _, s) => { s.regs[r] /= 2; s.pc++; },
      [Instruct.Tpl] = (r, _, s) => { s.regs[r] *= 3; s.pc++; },
      [Instruct.Inc] = (r, _, s) => { s.regs[r] += 1; s.pc++; },
      [Instruct.Jmp] = (_, o, s) => s.pc += o,
      [Instruct.Jie] = (r, o, s) => s.pc += s.regs[r] % 2 == 0 ? o : 1,
      [Instruct.Jio] = (r, o, s) => s.pc += s.regs[r] == 1 ? o : 1,
    };

    public void Run() {
      while (Running) {
        var inst = Insts[pc];
        _actions[inst.ins](inst.reg, inst.offset, this);
      }
    }

    internal void Set(string reg, int v) =>
      regs[reg] = v;
  }

  public int Part1(string result) {
    var inst = Lines().Select(Inst.Parse).ToArray();

    var state = new State(inst);

    state.Run();

    return state[result];
  }
  public string Part1Result() =>
    $"{Part1("b")}";

  public int Part2() {
    var inst = Lines().Select(Inst.Parse).ToArray();

    var state = new State(inst);
    state.Set("a", 1);

    state.Run();

    return state["b"];
  }
  public string Part2Result() =>
    $"{Part2()}";
}