use crate::shared::input_lines;

pub fn run(suffix: &str) {
    let lines = input_lines("day02", suffix);

    let result = part1(&lines);
    println!("Part1: {}", result);

    let result = part2(&lines);
    println!("Part2: {}", result);
}

fn part2(lines: &Vec<String>) -> String {
    todo!()
}

fn part1(lines: &Vec<String>) -> String {
    todo!()
}


mod tests {
    #[allow(unused_imports)]
    use super::{part1, part2, input_lines};

    #[test]
    fn first() {
        let lines = input_lines("day02", ".sample");

        let result = part1(&lines);

        assert_eq!(result, "1985");
    }
}