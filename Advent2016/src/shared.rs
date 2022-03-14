pub mod pt;
pub mod timing;

use std::fmt::Debug;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::str::FromStr;

fn input_path(prefix: &str, suffix: &str) -> PathBuf {
    PathBuf::from("input").join(prefix.to_owned() + suffix)
}

// Returns an Iterator to the Reader of the lines of the file.
pub fn input_lines(prefix: &str, suffix: &str) -> Vec<String> {
    let file = File::open(input_path(prefix, suffix));
    let lines = io::BufReader::new(file.unwrap()).lines();
    lines.map(|l| l.unwrap()).collect()
}

pub fn input_string(prefix: &str, suffix: &str) -> String {
    read_to_string(input_path(prefix, suffix)).unwrap()
}

pub fn input_parse<E: Debug, T: FromStr<Err = E>>(prefix: &str, suffix: &str) -> Vec<T> {
    let file = File::open(input_path(prefix, suffix));
    let lines = io::BufReader::new(file.unwrap()).lines();
    lines
        .filter_map(|l| l.unwrap().parse::<T>().ok())
        .collect()
}

pub fn vec_parse<E: Debug, O: FromStr<Err = E>>(vec: &Vec<&str>) -> Vec<O> {
    vec.iter()
        .map(|&l| l.parse::<O>().unwrap())
        .collect()
}
