use common::read_input;

#[derive(Debug)]
struct Rule(u32, u32);

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (min, max) = value.trim().split_once('-').unwrap();
        Self(min.parse().unwrap(), max.parse().unwrap())
    }
}

impl Rule {
    fn is_valid(&self, address: u32) -> bool {
        address < self.0 || address > self.1
    }
}

#[derive(Debug)]
struct Rules(Vec<Rule>);

impl From<&str> for Rules {
    fn from(value: &str) -> Self {
        let rules = value.trim().lines().map(|l| l.into()).collect();
        Self(rules)
    }
}

impl Rules {
    fn is_valid(&self, address: u32) -> bool {
        self.0.iter().all(|rule| rule.is_valid(address))
    }
}

fn find_min_valid(min: u32, max: u32, rules: &Rules) -> u32 {
    (min..max)
        .skip_while(|address| !rules.is_valid(*address))
        .next()
        .unwrap()
}

fn main() {
    let input = read_input("day20.txt");
    println!(
        "Part 1 = {}",
        find_min_valid(0, u32::max_value(), &input.as_str().into())
    );
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"5-8
0-2
4-7"#;
        let rules = input.into();
        assert_eq!(find_min_valid(0, 9, &rules), 3);
    }
}
