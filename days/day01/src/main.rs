use common::read_input;

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
        }
    }
}

enum Instruction {
    TurnLeft,
    TurnRight,
    Walk,
}

impl Instruction {
    fn from_str(value: &str) -> Vec<Self> {
        let (turn, movement) = value.trim().split_at(1);
        let mut instructions = match turn {
            "L" => vec![Self::TurnLeft],
            "R" => vec![Self::TurnRight],
            c => panic!("Unknown turn command {c}"),
        };
        (0..movement.parse().unwrap()).for_each(|_| instructions.push(Self::Walk));
        instructions
    }
}

struct Walker {
    position: (i32, i32),
    direction: Direction,
}

impl Walker {
    fn new() -> Self {
        Walker {
            position: (0, 0),
            direction: Direction::North,
        }
    }

    fn step(&self) -> (i32, i32) {
        match self.direction {
            Direction::North => (self.position.0, self.position.1 - 1),
            Direction::East => (self.position.0 + 1, self.position.1),
            Direction::South => (self.position.0, self.position.1 + 1),
            Direction::West => (self.position.0 - 1, self.position.1),
        }
    }

    fn execute(
        mut self,
        instructions: &[Instruction],
        mut visited: Vec<(i32, i32)>,
        stop_at_same_location: bool,
    ) -> Self {
        if instructions.is_empty() {
            return self;
        }

        let (i, rest) = instructions.split_first().unwrap();
        let moved = match i {
            Instruction::TurnLeft => {
                self.direction = self.direction.left();
                false
            }
            Instruction::TurnRight => {
                self.direction = self.direction.right();
                false
            }
            Instruction::Walk => {
                self.position = self.step();
                true
            }
        };
        if moved && stop_at_same_location && visited.contains(&self.position) {
            return self;
        }
        visited.push(self.position);

        return self.execute(rest, visited, stop_at_same_location);
    }

    fn get_distance(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }
}

fn main() {
    let input = read_input("day01.txt");
    let instructions = input.split(", ").fold(vec![], |mut instructions, i| {
        instructions.extend(Instruction::from_str(i));
        instructions
    });
    let mut walker = Walker::new();
    walker = walker.execute(&instructions, vec![], false);
    println!("Part1 = {}", walker.get_distance());
    let mut walker = Walker::new();
    walker = walker.execute(&instructions, vec![], true);
    println!("Part2 = {}", walker.get_distance());
}

#[cfg(test)]
mod day01_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "R2, L3", "R2, R2, R2", "R5, L5, R5, R3" },
        expected = { 5, 2, 12 }
    )]
    fn part1(input: &str, expected: i32) {
        let instructions = input.split(", ").fold(vec![], |mut instructions, i| {
            instructions.extend(Instruction::from_str(i));
            instructions
        });
        let mut walker = Walker::new();
        walker = walker.execute(&instructions, vec![], false);
        assert_eq!(walker.get_distance(), expected);
    }

    #[test]
    fn part2() {
        let input = "R8, R4, R4, R8";
        let instructions = input.split(", ").fold(vec![], |mut instructions, i| {
            instructions.extend(Instruction::from_str(i));
            instructions
        });
        let mut walker = Walker::new();
        walker = walker.execute(&instructions, vec![], true);
        assert_eq!(walker.get_distance(), 4);
    }
}
