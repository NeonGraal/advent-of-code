use std::{cmp::Ordering, convert::Infallible, fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord)]
pub enum Equip {
    Chip(String),
    Gen(String),
}

impl fmt::Display for Equip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Equip::Chip(n) => write!(f, "{}-C", n),
            Equip::Gen(n) => write!(f, "{}-G", n),
        }
    }
}

impl PartialOrd for Equip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_string()
            .partial_cmp(&other.to_string())
    }
}

impl FromStr for Equip {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s[..2].to_string();
        Ok(if s.contains("-compatible") {
            Equip::Chip(name)
        } else {
            Equip::Gen(name)
        })
    }
}

impl Equip {
    fn chip_name(&self) -> Option<&String> {
        match self {
            Equip::Chip(n) => Some(n),
            _ => None,
        }
    }

    fn gen_eq(&self, name: &String) -> bool {
        match self {
            Equip::Gen(n) => n == name,
            _ => false,
        }
    }

    fn gen_neq(&self, name: &String) -> bool {
        match self {
            Equip::Gen(n) => n != name,
            _ => false,
        }
    }
}
mod tests {
    #[allow(unused_imports)]
    use super::{Equip, FromStr};

    #[test]
    fn parse_gen() {
        let equip = Equip::from_str("Alpha").unwrap();

        assert_eq!(equip, Equip::Gen("Al".to_string()));
    }

    #[test]
    fn parse_chip() {
        let equip = Equip::from_str("Alpha-compatible").unwrap();

        assert_eq!(equip, Equip::Chip("Al".to_string()));
    }
}
