use crate::shared::{input_string, timing::Timing};

use md5;

const DAY_NAME: &str = "day05";

pub fn run(suffix: &str) {
    println!("** Day 5");

    let s = input_string(DAY_NAME, suffix);

    let result = part1(&s);
    println!("- Part1: {}", result);

    let result = part2(&s);
    println!("- Part2: {}", result);
}

fn hashes(salt: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    let mut t = Timing::start(100, 5);

    let mut i = 0;

    let collector = std::iter::from_fn(move || {
        let mut result: [u8; 16] = [1; 16];

        while result[0] != 0 || result[1] != 0 || result[2] > 15 {
            let data = format!("{}{}", salt, i);
            i = i + 1;
            let digest = md5::compute(data.to_string());
            result = <[u8; 16]>::from(digest);
            t.check(|| format!("{}", i));
        }
        print!("!");

        Some((result[2], result[3]))
    });

    collector
}

fn part1(lines: &String) -> String {
    let s: String = hashes(lines)
        .take(8)
        .map(|r| format!("{:x}", r.0))
        .collect();

    println!("");

    return s;
}

fn part2(lines: &String) -> String {
    let mut s = ['_'; 8];

    for (pos, digit) in hashes(lines) {
        let i = pos as usize;
        if i < 8 && s[i] == '_' {
            s[i] = format!("{:02x}", digit)
                .chars()
                .next()
                .unwrap();
        }
        if s.iter().all(|&c| c != '_') {
            println!("");
            return s.iter().collect();
        }
    }

    println!("");
    "".to_string()
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_string, part1, part2, DAY_NAME};

    #[test]
    fn first() {
        let lines = input_string(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, "18f47a30");
    }

    #[test]
    fn second() {
        let lines = input_string(DAY_NAME, ".sample");

        let result = part2(&lines);

        assert_eq!(result, "05ace8e3");
    }
}
