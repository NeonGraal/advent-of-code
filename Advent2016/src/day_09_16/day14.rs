use std::{collections::HashMap, fmt, time::{Instant, Duration}};

use crate::shared::input_string;

const DAY_NAME: &str = "day14";

pub fn run(suffix: &str) {
    println!("** Day 14 ");

    let s = input_string(DAY_NAME, suffix);

    let result = part1(&s);
    println!("- Part1: {}", result);

    let result = part2(&s);
    println!("- Part2: {}", result);
}

fn triple(chars: &Vec<char>) -> Option<char> {
    let mut iter = chars.iter();
    let mut a = [*iter.next().unwrap(), *iter.next().unwrap()];

    for &l in iter {
        if a[0] == a[1] && a[0] == l {
            return Some(l);
        }
        a = [a[1], l];
    }
    None
}

fn has_five(chars: &Vec<char>, of: char) -> bool {
    let mut count = 0;
    for &c in chars {
        if c == of {
            count = count + 1;
            if count == 5 {
                return true;
            }
        } else {
            count = 0;
        }
    }
    false
}

#[derive(Debug)]
struct Key(usize, Vec<char>, usize, Vec<char>);

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} : {} ({} : {})",
            self.0,
            String::from_iter(&self.1),
            self.2,
            String::from_iter(&self.3)
        )
    }
}

fn keys<'a, F>(mut hash: F) -> impl Iterator<Item = Key> + 'a
where
    F: 'a,
    F: FnMut(usize) -> Vec<char>,
{
    let start = Instant::now();
    let second = Duration::from_millis(500);
    let minute = Duration::from_secs(30);
    let mut secs = Instant::now();
    let mut mins = Instant::now();
    
    let mut i = 0_usize;
    let mut cnt = 0_usize;
    let mut hashes = HashMap::<usize, Vec<char>>::new();

    for h in 0..1000 {
        hashes.insert(h, hash(h));
    }

    let collector = std::iter::from_fn(move || loop {
        if secs.elapsed() > second {
            secs = Instant::now();
            print!(".");
            if mins.elapsed() > minute {
                mins = Instant::now();
                println!(" {}s {}/{}", start.elapsed().as_secs(), i, cnt);
            }
        }
        let result = hashes.remove(&i).unwrap();
        i = i + 1;
        hashes.insert(i + 999, hash(i + 999));
        if let Some(fives) = triple(&result) {
            for j in i..i + 1000 {
                if has_five(&hashes[&j], fives) {
                    cnt = cnt + 1;
                    print!("!");
                    return Some(Key(i - 1, result, j, hashes[&j].clone()));
                }
            }
        }
    });

    collector
}

fn hash(salt: &str, idx: usize) -> Vec<char> {
    let data = format!("{}{}", salt, idx);
    let digest = md5::compute(data.to_string());
    format!("{:x}", digest)
        .chars()
        .collect()
}

fn part1(lines: &String) -> usize {
    let mut keys = keys(|i| hash(lines, i));
    let key = keys.nth(63).unwrap();

    println!("\n{} - 64 - {}", lines, key);
    key.0
}

fn stretched(salt: &str, idx: usize) -> Vec<char> {
    let data = format!("{}{}", salt, idx);
    let mut digest = md5::compute(data.to_string());
    let mut hash = format!("{:x}", digest);
    for i in 0..2016 {
        digest = md5::compute(hash.to_string());
        hash = format!("{:x}", digest);
    }
    hash.chars().collect()
}

fn part2(lines: &String) -> usize {
    let mut keys = keys(|i| stretched(lines, i));
    let key = keys.nth(63).unwrap();

    println!("\n{} - 64 - {}", lines, key);
    key.0
}

mod tests {
    #[allow(unused_imports)]
    use super::{hash, keys, part1, part2, stretched, DAY_NAME};

    #[test]
    fn first_triple() {
        let chars = hash("abc", 18);
        let result = String::from_iter(chars);

        assert!(result.contains("cc38887a5"));
    }

    #[test]
    fn first_key() {
        let mut keys = keys(|i| hash("abc", i));

        let key = keys.next().unwrap();
        println!("{}", key);
        assert_eq!(key.0, 39);

        let key = keys.next().unwrap();
        println!("{}", key);
        assert_eq!(key.0, 92);
    }

    #[test]
    fn first() {
        let result = part1(&"abc".to_string());

        assert_eq!(result, 22728);
    }

    #[test]
    fn second_triple() {
        let chars = stretched("abc", 5);
        let result = String::from_iter(chars);

        assert!(result.contains("222"));
    }

    #[test]
    fn second_key() {
        let mut keys = keys(|i| stretched("abc", i));

        let key = keys.next().unwrap();
        println!("{}", key);
        assert_eq!(key.0, 10);
    }

    #[test]
    fn second() {
        let result = part2(&"abc".to_string());

        assert_eq!(result, 22551);
    }
}
