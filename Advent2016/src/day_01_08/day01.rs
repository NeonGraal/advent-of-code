use crate::shared;

pub fn run(suffix: &str) {
    let prefix = "day01".to_owned();
    let lines = shared::read_lines(prefix + suffix);
    let lines = match lines {
        Ok(l) => l,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    for line in lines {
        if let Ok(first) = line {
            println!("First: {}", first);
        }
        break;
    }
}
