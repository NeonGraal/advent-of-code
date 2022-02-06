pub mod pt;

use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::PathBuf;

fn input_path(prefix: &str, suffix: &str) -> PathBuf {
    PathBuf::from("input").join(prefix.to_owned() + suffix)
}

// Returns an Iterator to the Reader of the lines of the file.
pub fn input_lines(prefix: &str, suffix: &str) -> Vec<String> {
    let file = File::open(input_path(prefix, suffix));
    let lines = match file {
        Ok(f) => io::BufReader::new(f).lines(),
        Err(error) => panic!("Problem reading the lines of {}{}: {:?}", prefix, suffix, error),
    };
    lines.map(|l| match l {
        Ok(s) => s,
        Err(error) => panic!("Problem reading the lines of day02{}: {:?}", suffix, error),
    }).collect()
}

pub fn input_string(prefix: &str, suffix: &str) -> String {
    let s = read_to_string(input_path(prefix, suffix));
    match s {
        Ok(s) => return s,
        Err(error) => panic!("Problem reading the whole file {}{}: {:?}", prefix, suffix, error),
    };
}
