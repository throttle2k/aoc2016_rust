use std::collections::VecDeque;

use common::read_input;

#[derive(Debug)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnPosition(char),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
}

fn parse_swap_position(input: &[&str]) -> Operation {
    let from = input.first().unwrap().parse().unwrap();
    let to = input.last().unwrap().parse().unwrap();
    Operation::SwapPosition(from, to)
}

fn parse_swap_letter(input: &[&str]) -> Operation {
    let from = input.first().unwrap().chars().next().unwrap();
    let to = input.last().unwrap().chars().next().unwrap();
    Operation::SwapLetter(from, to)
}

fn parse_swap(input: &[&str]) -> Operation {
    match *input.first().unwrap() {
        "position" => parse_swap_position(&input[1..]),
        "letter" => parse_swap_letter(&input[1..]),
        s => panic!("Unknown swap operation: {s}"),
    }
}

fn parse_rotate_left(input: &[&str]) -> Operation {
    let of = input.first().unwrap().parse().unwrap();
    Operation::RotateLeft(of)
}

fn parse_rotate_right(input: &[&str]) -> Operation {
    let of = input.first().unwrap().parse().unwrap();
    Operation::RotateRight(of)
}

fn parse_rotate_based(input: &[&str]) -> Operation {
    let base = input.last().unwrap().chars().next().unwrap();
    Operation::RotateBasedOnPosition(base)
}

fn parse_rotate(input: &[&str]) -> Operation {
    match *input.first().unwrap() {
        "left" => parse_rotate_left(&input[1..]),
        "right" => parse_rotate_right(&input[1..]),
        "based" => parse_rotate_based(&input[1..]),
        s => panic!("Unknown rotate operation: {s}"),
    }
}

fn parse_reverse(input: &[&str]) -> Operation {
    let from = input.iter().skip(1).next().unwrap().parse().unwrap();
    let to = input.last().unwrap().parse().unwrap();
    Operation::ReversePositions(from, to)
}

fn parse_move(input: &[&str]) -> Operation {
    let from = input.iter().skip(1).next().unwrap().parse().unwrap();
    let to = input.last().unwrap().parse().unwrap();
    Operation::MovePosition(from, to)
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let input = value.split_whitespace().collect::<Vec<_>>();
        match *input.first().unwrap() {
            "swap" => parse_swap(&input[1..]),
            "rotate" => parse_rotate(&input[1..]),
            "reverse" => parse_reverse(&input[1..]),
            "move" => parse_move(&input[1..]),
            s => panic!("Unknown operation: {s}"),
        }
    }
}

impl Operation {
    fn apply(&self, input: &str, reverse: bool) -> String {
        let mut queue = input.chars().collect::<VecDeque<_>>();
        match (self, reverse) {
            (Operation::SwapPosition(from, to), _) => {
                queue.swap(*from, *to);
            }
            (Operation::SwapLetter(from_char, to_char), _) => {
                let from = queue.iter().position(|c| c == from_char).unwrap();
                let to = queue.iter().position(|c| c == to_char).unwrap();
                queue.swap(from, to)
            }
            (Operation::RotateLeft(of), false) => queue.rotate_left(*of),
            (Operation::RotateLeft(of), true) => queue.rotate_right(*of),
            (Operation::RotateRight(of), false) => queue.rotate_right(*of),
            (Operation::RotateRight(of), true) => queue.rotate_left(*of),
            (Operation::RotateBasedOnPosition(of), false) => {
                let index = queue.iter().position(|c| c == of).unwrap();
                queue.rotate_right(index + 1);
                if index >= 4 {
                    queue.rotate_right(1);
                }
            }
            (Operation::RotateBasedOnPosition(_), true) => {
                queue = (0..queue.len())
                    .map(|index| {
                        let mut new_queue = queue.clone();
                        new_queue.rotate_left(index);
                        new_queue.iter().collect::<String>()
                    })
                    .skip_while(|new_input| self.apply(new_input.as_str(), false) != input)
                    .next()
                    .unwrap()
                    .chars()
                    .collect::<VecDeque<_>>();
            }
            (Operation::ReversePositions(from, to), _) => {
                let half = (to - from) / 2;
                (0..=half).for_each(|delta| queue.swap(from + delta, to - delta));
            }
            (Operation::MovePosition(from, to), false) => {
                let c = queue.remove(*from).unwrap();
                queue.insert(*to, c);
            }
            (Operation::MovePosition(from, to), true) => {
                let c = queue.remove(*to).unwrap();
                queue.insert(*from, c);
            }
        }
        queue.iter().collect()
    }
}

struct Operations(Vec<Operation>);

impl From<&str> for Operations {
    fn from(value: &str) -> Self {
        Self(value.trim().lines().map(|l| l.trim().into()).collect())
    }
}

impl Operations {
    fn apply(&self, input: &str) -> String {
        self.0
            .iter()
            .fold(input.to_owned(), |s, op| op.apply(&s, false))
    }

    fn apply_reverse(&self, input: &str) -> String {
        self.0.iter().rev().fold(input.to_owned(), |s, op| {
            let new_s = op.apply(&s, true);
            assert_eq!(s, op.apply(new_s.as_str(), false));
            new_s
        })
    }
}

fn main() {
    let input = read_input("day21.txt");
    let operations = Operations::from(input.as_str());
    println!("Part 1 = {}", operations.apply("abcdefgh"));
    println!("Part 2 = {}", operations.apply_reverse("fbgdceah"));
}

#[cfg(test)]
mod day21_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "abcde", "ebcda", "edcba", "abcde", "bcdea", "bdeac", "abdec", "ecabd" },
        operation = { "swap position 4 with position 0", "swap letter d with letter b", "reverse positions 0 through 4", "rotate left 1 step", "move position 1 to position 4", "move position 3 to position 0", "rotate based on position of letter b", "rotate based on position of letter d" },
        expected = { "ebcda", "edcba", "abcde", "bcdea", "bdeac", "abdec", "ecabd", "decab" },
    )]
    fn test_apply_operation(input: &str, operation: &str, expected: &str) {
        let operation = Operation::from(operation);
        assert_eq!(operation.apply(input, false), expected);
    }

    #[test]
    fn part1() {
        let input = r#"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d"#;
        let operations = Operations::from(input);
        assert_eq!(operations.apply("abcde"), "decab".to_owned());
    }

    #[parameterized(
        input = { "ebcda", "edcba", "abcde", "bcdea", "bdeac", "abdec", "ecabd", "decab" },
        operation = { "swap position 4 with position 0", "swap letter d with letter b", "reverse positions 0 through 4", "rotate left 1 step", "move position 1 to position 4", "move position 3 to position 0", "rotate based on position of letter b", "rotate based on position of letter d" },
        expected = { "abcde", "ebcda", "edcba", "abcde", "bcdea", "bdeac", "abdec", "ecabd" },
    )]
    fn test_apply_operation_reversed(input: &str, operation: &str, expected: &str) {
        let operation = Operation::from(operation);
        assert_eq!(operation.apply(input, true), expected);
    }

    #[test]
    fn part2() {
        let input = r#"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d"#;
        let operations = Operations::from(input);
        assert_eq!(operations.apply_reverse("decab"), "abcde".to_owned());
    }
}
