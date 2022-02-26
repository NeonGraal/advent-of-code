use std::{num::ParseIntError, str::FromStr, u32};

use crate::shared::input_parse;

const DAY_NAME: &str = "day20";

pub fn run(suffix: &str) {
    println!("** Day 20");

    let ranges = input_parse(DAY_NAME, suffix);

    let result = part1(&ranges, u32::MAX);
    println!("- Part1: {}", result);

    let result = part2(&ranges, u32::MAX);
    println!("- Part2: {}", result);
}

fn part1(ranges: &Vec<Range>, max: u32) -> u32 {
    let mut valid = vec![Range { from: 0, to: max }];

    for r in ranges {
        r.remove_from(&mut valid);
    }

    valid[0].from
}

fn part2(ranges: &Vec<Range>, max: u32) -> u32 {
    let mut valid = vec![Range { from: 0, to: max }];

    for r in ranges {
        r.remove_from(&mut valid);
    }

    valid
        .iter()
        .map(|r| r.to - r.from + 1)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    from: u32,
    to: u32,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");
        let from = parts.next().unwrap().parse::<u32>()?;
        let to = parts.next().unwrap().parse::<u32>()?;
        Ok(Range { from, to })
    }
}

impl Range {
    fn remove_from(&self, valid: &mut Vec<Range>) {
        let len = valid.len();
        let mut start = 0;
        while start < len && self.from > valid[start].to {
            start = start + 1;
        }
        if start == len {
            return;
        }
        let mut end = start;
        while end < len && self.to >= valid[end].from {
            end = end + 1;
        }
        if start == end {
            return;
        }
        let item = valid[start];
        if self.from > item.from {
            if start + 1 == end && self.to < item.to {
                let from = self.to + 1;
                let element = Range { from, to: item.to };
                valid.insert(end, element);
            }
            valid[start].to = self.from - 1;
            start = start + 1;
        }
        end = end - 1;
        if self.to < valid[end].to {
            valid[end].from = self.to + 1;
            if end == 0 {
                return;
            }
            end = end - 1;
        }
        while end >= start {
            valid.remove(end);
            if end == 0 {
                return;
            }
            end = end - 1;
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_parse, part1, part2, Range, DAY_NAME};

    #[test]
    fn remove_before() {
        let expected = Range { from: 10, to: 20 };
        let mut valid = vec![expected.clone()];
        let r = Range { from: 2, to: 8 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0], expected);
    }

    #[test]
    fn remove_after() {
        let expected = Range { from: 10, to: 20 };
        let mut valid = vec![expected.clone()];
        let r = Range { from: 22, to: 28 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0], expected);
    }

    #[test]
    fn remove_from() {
        let mut valid = vec![Range { from: 10, to: 20 }];
        let r = Range { from: 5, to: 15 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0], Range { from: 16, to: 20 });
    }

    #[test]
    fn remove_from_exactly() {
        let mut valid = vec![Range { from: 10, to: 20 }];
        let r = Range { from: 10, to: 15 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0], Range { from: 16, to: 20 });
    }

    #[test]
    fn remove_to() {
        let mut valid = vec![Range { from: 10, to: 20 }];
        let r = Range { from: 15, to: 25 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0], Range { from: 10, to: 14 });
    }

    #[test]
    fn remove_to_exactly() {
        let mut valid = vec![Range { from: 10, to: 20 }];
        let r = Range { from: 15, to: 20 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0], Range { from: 10, to: 14 });
    }

    #[test]
    fn remove_around() {
        let mut valid = vec![Range { from: 10, to: 20 }];
        let r = Range { from: 5, to: 25 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 0);
    }

    #[test]
    fn remove_around_exactly() {
        let mut valid = vec![Range { from: 10, to: 20 }];
        let r = Range { from: 10, to: 20 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 0);
    }

    #[test]
    fn remove_in() {
        let mut valid = vec![Range { from: 10, to: 20 }];
        let r = Range { from: 14, to: 16 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 2);
        assert_eq!(valid[0], Range { from: 10, to: 13 });
        assert_eq!(valid[1], Range { from: 17, to: 20 });
    }

    #[test]
    fn remove_across() {
        let mut valid = vec![Range { from: 10, to: 20 }, Range { from: 30, to: 40 }];
        let r = Range { from: 15, to: 35 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 2);
        assert_eq!(valid[0], Range { from: 10, to: 14 });
        assert_eq!(valid[1], Range { from: 36, to: 40 });
    }

    #[test]
    fn remove_across_more() {
        let mut valid = vec![
            Range { from: 10, to: 20 },
            Range { from: 22, to: 28 },
            Range { from: 30, to: 40 },
        ];
        let r = Range { from: 15, to: 35 };

        r.remove_from(&mut valid);

        assert_eq!(valid.len(), 2);
        assert_eq!(valid[0], Range { from: 10, to: 14 });
        assert_eq!(valid[1], Range { from: 36, to: 40 });
    }

    #[test]
    fn first() {
        let ranges = input_parse(DAY_NAME, ".sample");

        let result = part1(&ranges, 9);

        assert_eq!(result, 3);
    }

    #[test]
    fn second() {
        let ranges = input_parse(DAY_NAME, ".sample");

        let result = part2(&ranges, 9);

        assert_eq!(result, 2);
    }
}
