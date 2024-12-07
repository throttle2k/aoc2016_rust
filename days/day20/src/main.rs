use common::read_input;

#[derive(Debug, Clone)]
struct Rule {
    start: u32,
    end: u32,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (min, max) = value.trim().split_once('-').unwrap();
        Self {
            start: min.parse().unwrap(),
            end: max.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Rules(Vec<Rule>);

impl From<&str> for Rules {
    fn from(value: &str) -> Self {
        let mut rules = value
            .trim()
            .lines()
            .map(|l| Rule::from(l))
            .collect::<Vec<_>>();
        rules.sort_by_key(|r| r.start);
        let rules = rules.iter().fold(Vec::<Rule>::new(), |mut merged, r| {
            if let Some(last) = merged.last_mut() {
                if last.end >= r.start {
                    last.end = last.end.max(r.end);
                } else {
                    merged.push(r.clone());
                }
            } else {
                merged.push(r.clone());
            }
            merged
        });

        Self(rules)
    }
}

impl Rules {
    fn min_valid(&self) -> u32 {
        self.0
            .windows(2)
            .skip_while(|rules| rules[0].end == rules[1].start - 1)
            .next()
            .unwrap()[0]
            .end
            + 1
    }

    fn count_valid(&self) -> u32 {
        self.0
            .windows(2)
            .map(|rules| rules[1].start - rules[0].end - 1)
            .sum()
    }
}

fn main() {
    let input = read_input("day20.txt");
    let rules = Rules::from(input.as_str());
    println!("Part 1 = {}", rules.min_valid());
    println!("Part 2 = {}", rules.count_valid());
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"5-8
0-2
4-7"#;
        let rules = Rules::from(input);
        assert_eq!(rules.min_valid(), 3);
    }
}
