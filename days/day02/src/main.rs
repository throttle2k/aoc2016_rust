use core::panic;
use std::{char, isize};

use common::read_input;

enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            c => panic!("Unknown move {c}"),
        }
    }
}

#[derive(Debug)]
struct Keypad {
    size: usize,
    keys: Vec<char>,
}

impl Keypad {
    fn new(size: usize) -> Self {
        Self {
            size,
            keys: (1..=size * size)
                .map(|r| char::from_digit(r as u32, 10).unwrap())
                .collect(),
        }
    }

    fn move_from(&self, from_key: &char, movements: &[Move]) -> char {
        if movements.is_empty() {
            return *from_key;
        }

        let (movement, rest) = movements.split_first().unwrap();
        let idx = self.keys.iter().position(|c| c == from_key).unwrap() as isize;
        let (next_idx, can_move) = match movement {
            Move::Up => {
                let next_idx = idx - self.size as isize;
                let out_of_bounds_up = next_idx < 0;
                (next_idx, !out_of_bounds_up)
            }
            Move::Right => {
                let next_idx = idx + 1;
                let out_of_bounds_side = next_idx / self.size as isize != idx / self.size as isize;
                (next_idx, !out_of_bounds_side)
            }
            Move::Down => {
                let next_idx = idx + self.size as isize;
                let out_of_bounds_bottom = next_idx >= (self.size * self.size) as isize;
                (next_idx, !out_of_bounds_bottom)
            }
            Move::Left => {
                let next_idx = idx - 1;
                let out_of_bounds_side =
                    next_idx < 0 || next_idx / self.size as isize != idx / self.size as isize;
                (next_idx, !out_of_bounds_side)
            }
        };
        if can_move {
            self.move_from(self.keys.get(next_idx as usize).unwrap(), rest)
        } else {
            self.move_from(from_key, rest)
        }
    }

    fn get_code(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| Move::from(c))
                    .collect::<Vec<_>>()
            })
            .fold(String::new(), |mut code, movements| {
                let next_digit = self.move_from(&code.chars().last().unwrap_or('5'), &movements);
                code.push(next_digit);
                code
            })
    }
}

fn main() {
    let input = read_input("day02.txt");
    let keypad = Keypad::new(3);
    println!("Part 1: {}", keypad.get_code(input.as_str()));
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"ULL
RRDDD
LURDL
UUUUD"#;
        let keypad = Keypad::new(3);
        assert_eq!(keypad.get_code(input), "1985");
    }
}
