use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::PathBuf;

fn input_path(prefix: &str, suffix: &str) -> PathBuf {
    PathBuf::from("input").join(prefix.to_owned() + suffix)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
#[allow(dead_code)]
pub fn input_lines(prefix: &str, suffix: &str) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(input_path(prefix, suffix))?;
    Ok(io::BufReader::new(file).lines())
}

pub fn input_string(prefix: &str, suffix: &str) -> io::Result<String>
{
    read_to_string(input_path(prefix, suffix))
}
