use std::str::Chars;

use crate::shared::input_string;

const DAY_NAME: &str = "day09";

pub fn run(suffix: &str) {
    println!("** Day 9");

    let s = input_string(DAY_NAME, suffix);

    let result = part1(&s);
    println!("- Part1: {}", result);

    let result = part2(&s);
    println!("- Part2: {}", result);
}

fn take_num(iter: &mut Chars, til: char) -> usize {
    let mut result: usize = 0;

    while let Some(c) = iter.next() {
        if c == til {
            return result;
        }
        match c.to_digit(10) {
            Some(n) => result = result * 10 + (n as usize),
            None => panic!("Invalid digit {}", c),
        }
    }
    0
}

fn decompress(s: &String) -> String {
    let mut iter = s.chars();
    let mut result = "".to_string();

    while let Some(c) = iter.next() {
        if c == '(' {
            let len = take_num(&mut iter, 'x');
            let cnt = take_num(&mut iter, ')');

            let mut sub = "".to_string();
            for i in 0..len {
                if let Some(s) = iter.next() {
                    sub.push(s);
                }
            }
            result = result + &sub.repeat(cnt);
        } else if !c.is_whitespace() {
            result.push(c);
        }
    }

    result
}

fn part1(s: &String) -> usize {
    decompress(s).chars().count()
}

fn decompress_len(s: &String) -> usize {
    let mut iter = s.chars();
    let mut result = 0;

    while let Some(c) = iter.next() {
        if c == '(' {
            let len = take_num(&mut iter, 'x');
            let cnt = take_num(&mut iter, ')');

            let mut sub = "".to_string();
            for i in 0..len {
                if let Some(s) = iter.next() {
                    sub.push(s);
                }
            }
            result = result + decompress_len(&sub) * cnt;
        } else if !c.is_whitespace() {
            result = result + 1;
        }
    }

    result
}

fn part2(s: &String) -> usize {
    decompress_len(s)
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_string, part1, part2, DAY_NAME};

    #[test]
    fn first() {
        assert_eq!(part1(&"ADVENT".to_string()), 6);
    }

    #[test]
    fn second() {
        assert_eq!(part1(&"A(1x5)BC".to_string()), 7);
    }

    #[test]
    fn third() {
        assert_eq!(part1(&"(3x3)XYZ".to_string()), 9);
    }

    #[test]
    fn fourth() {
        assert_eq!(part1(&"A(2x2)BCD(2x2)EFG".to_string()), 11);
    }

    #[test]
    fn fifth() {
        assert_eq!(part1(&"(6x1)(1x3)A".to_string()), 6);
    }

    #[test]
    fn sixth() {
        assert_eq!(part1(&"X(8x2)(3x3)ABCY".to_string()), 18);
    }

    #[test]
    fn seventh() {
        assert_eq!(part2(&"(3x3)XYZ".to_string()), 9);
    }

    #[test]
    fn eighth() {
        assert_eq!(part2(&"X(8x2)(3x3)ABCY".to_string()), 20);
    }

    #[test]
    fn ninth() {
        assert_eq!(
            part2(&"(27x12)(20x12)(13x14)(7x10)(1x12)A".to_string()),
            241920
        );
    }

    #[test]
    fn tenth() {
        assert_eq!(
            part2(&"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string()),
            445
        );
    }
}
// cspell:word ABCY PQRSTX
