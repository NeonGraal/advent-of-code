use crate::shared::input_lines;

const DAY_NAME: &str = "day07";

pub fn run(suffix: &str) {
    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("Part1: {}", result);

    let result = part2(&lines);
    println!("Part2: {}", result);
}

fn nets(line: &String) -> (Vec<String>, Vec<String>) {
    let mut super_net = Vec::new();
    let mut hyper_net = Vec::new();

    for (i, net) in line.split(&['[', ']']).enumerate() {
        if i % 2 == 0 {
            super_net.push(net.to_string());
        } else {
            hyper_net.push(net.to_string());
        }
    }

    (super_net, hyper_net)
}

fn has_abba(net: &String) -> bool {
    let mut chars = net.chars();
    let mut a = chars.next().unwrap();
    let mut b = chars.next().unwrap();
    let mut c = chars.next().unwrap();
    for d in chars {
        if a == d && b == c && a != b {
            return true;
        }
        a = b;
        b = c;
        c = d;
    }
    false
}

#[rustfmt::skip]
fn supports_tls(line: &String) -> bool {
    let (super_net, hyper_net) = nets(line);

    super_net.iter().any(|n| 
        has_abba(n)) 
        && hyper_net.iter().all(|n| !has_abba(n))
}

fn part1(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .filter(|&l| supports_tls(l))
        .count() as i32
}

fn find_abas(nets: Vec<String>) -> Vec<(char, char)> {
    let mut result = Vec::new();

    for net in nets {
        let mut chars = net.chars();

        let mut a = chars.next().unwrap();
        let mut b = chars.next().unwrap();

        for c in chars {
            if a == c && a != b {
                let item = (a, b);
                if !result.contains(&item) {
                    result.push(item);
                }
            }
            a = b;
            b = c;
        }
    }

    result
}

fn has_bab(net: &String, aba: &(char, char)) -> bool {
    let mut chars = net.chars();
    let mut a = chars.next().unwrap();
    let mut b = chars.next().unwrap();
    for c in chars {
        if a == c && b == aba.0 && a == aba.1 {
            return true;
        }
        a = b;
        b = c;
    }
    false
}

fn any_babs(net: &String, abas: &Vec<(char, char)>) -> bool {
    abas.iter().any(|aba| has_bab(net, aba))
}

fn supports_ssl(line: &String) -> bool {
    let (super_net, hyper_net) = nets(line);

    let abas = find_abas(super_net);
    hyper_net
        .iter()
        .any(|n| any_babs(n, &abas))
}

fn part2(lines: &Vec<String>) -> i32 {
    for line in lines {}
    lines
        .iter()
        .filter(|&l| supports_ssl(l))
        .count() as i32
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_lines, part1, part2, DAY_NAME};

    #[test]
    fn first() {
        let lines = input_lines(DAY_NAME, ".sample1");

        let result = part1(&lines);

        assert_eq!(result, 2);
    }

    #[test]
    fn second() {
        let lines = input_lines(DAY_NAME, ".sample2");

        let result = part2(&lines);

        assert_eq!(result, 3);
    }
}
