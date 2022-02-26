const DAY_NAME: &str = "day19";

pub fn run(suffix: &str) {
    println!("** Day 19");

    let elves = 3018458_usize;

    let result = part1(elves);
    println!("- Part1: {}", result);

    let result = part2(elves);
    println!("- Part2: {}", result);
}

fn part1(elves: usize) -> usize {
    let log = (elves as f64).log2();
    let base = 2_usize.pow(log as u32);

    (elves - base) * 2 + 1
}

fn part2(elves: usize) -> usize {
    let log = (elves as f64).log(3.0);
    let base = 3_usize.pow(log as u32);

    if elves == base {
        return elves;
    }
    if elves / base < 2 {
        return elves - base;
    }

    (elves - base * 2) * 2 + base
}

mod tests {
    #[allow(unused_imports)]
    use super::{part1, part2, DAY_NAME};

    fn simple1(count: usize) -> usize {
        let mut elves = Vec::from_iter(0..count);
        let mut i = 0;
        let mut len = count;

        while len > 1 {
            i = i + 1;
            if i == len {
                i = 0;
            }
            elves.remove(i);
            len = len - 1;
            if i == len {
                i = 0;
            }
        }
        elves[0] + 1
    }

    #[test]
    fn first() {
        let mut errors = false;
        for i in 2..30 {
            let expected = simple1(i);
            let result = part1(i);
            if expected != result {
                errors = true;
                println!("{}: {} != {}", i, result, expected);
            }
        }
        assert!(!errors);
    }

    fn simple2(count: usize) -> usize {
        let mut elves = Vec::from_iter(0..count);
        let mut i = 0;
        let mut len = count;

        while len > 1 {
            let remove = (i + len / 2) % len;
            elves.remove(remove);
            if remove > i {
                i = i + 1;
            }
            len = len - 1;
            if i >= len {
                i = 0;
            }
        }
        elves[0] + 1
    }

    #[test]
    fn second() {
        let mut errors = false;
        for i in 2..30 {
            let expected = simple2(i);
            let result = part2(i);
            if expected != result {
                errors = true;
                println!("{}: {} != {}", i, result, expected);
            }
        }
        assert!(!errors);
    }
}
