use crate::shared::input_lines;

const DAY_NAME: &str = "day03";

pub fn run(suffix: &str) {
    println!("** Day 3");

    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut count = 0;

    for tri in lines.chunks(3) {
        let nums: Vec<Vec<i32>> = tri
            .into_iter()
            .map(|l| make_triangle(&l))
            .collect();

        for i in 0..3 {
            if valid_triangle(nums[0][i], nums[1][i], nums[2][i]) {
                count += 1;
            }
        }
    }

    count
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut count = 0;

    for nums in lines
        .into_iter()
        .map(|l| make_triangle(&l))
    {
        if valid_triangle(nums[0], nums[1], nums[2]) {
            count += 1;
        }
    }

    count
}

fn make_triangle(l: &str) -> Vec<i32> {
    l.split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn valid_triangle(s1: i32, s2: i32, s3: i32) -> bool {
    s1 < s2 + s3 && s2 < s3 + s1 && s3 < s1 + s2
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_lines, part1, part2, valid_triangle, DAY_NAME};

    #[test]
    fn first() {
        let result = valid_triangle(5, 10, 25);

        assert_eq!(result, false);
    }
}
