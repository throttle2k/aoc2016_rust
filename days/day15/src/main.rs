use common::read_input;

#[derive(Clone)]
struct Disc {
    id: usize,
    num_positions: usize,
    position: usize,
}

impl From<&str> for Disc {
    fn from(value: &str) -> Self {
        let mut splits = value.split_whitespace();
        splits.next();
        let id = splits
            .next()
            .unwrap()
            .strip_prefix('#')
            .unwrap()
            .parse()
            .unwrap();
        splits.next();
        let num_position = splits.next().unwrap().parse().unwrap();
        splits.next();
        splits.next();
        splits.next();
        splits.next();
        splits.next();
        splits.next();
        splits.next();
        let starting_position = splits
            .next()
            .unwrap()
            .strip_suffix('.')
            .unwrap()
            .parse()
            .unwrap();
        Self {
            id,
            num_positions: num_position,
            position: starting_position,
        }
    }
}

impl Disc {
    fn position_for_capsule(&self) -> usize {
        (((-(self.id as isize) % self.num_positions as isize) + self.num_positions as isize)
            % self.num_positions as isize) as usize
    }

    fn is_ready(&self) -> bool {
        self.position == self.position_for_capsule()
    }
}

#[derive(Clone)]
struct Machine {
    discs: Vec<Disc>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let discs = value
            .trim()
            .lines()
            .map(|l| l.trim().into())
            .collect::<Vec<_>>();
        Self { discs }
    }
}

impl Machine {
    fn tick(&self) -> Self {
        let discs = self
            .discs
            .iter()
            .map(|d| Disc {
                position: (d.position + 1) % d.num_positions,
                ..d.clone()
            })
            .collect();
        Self {
            discs,
            ..self.clone()
        }
    }

    fn is_ready(&self) -> bool {
        self.discs.iter().all(|d| d.is_ready())
    }

    fn seconds_for_capsule(&self) -> usize {
        let mut m = self.clone();
        let mut count = 0;
        while !m.is_ready() {
            m = m.tick();
            count += 1;
        }
        count
    }
}

fn main() {
    let input = read_input("day15.txt");
    let machine = Machine::from(input.as_str());
    println!("Part 1 = {}", machine.seconds_for_capsule());
}

#[cfg(test)]
mod day15_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        num_positions = { 1, 2, 5, 11 },
        id = { 3, 1, 4, 2 },
        expected = { 0, 1, 1, 9 }
    )]
    fn test_position_for_capsule(num_positions: usize, id: usize, expected: usize) {
        let disc = Disc {
            id,
            num_positions,
            position: 0,
        };
        assert_eq!(disc.position_for_capsule(), expected);
    }

    #[test]
    fn part1() {
        let input = r#"Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1."#;
        let machine = Machine::from(input);
        assert_eq!(machine.seconds_for_capsule(), 5);
    }
}
