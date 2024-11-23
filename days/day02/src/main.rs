use core::panic;
use std::{char, isize};

use common::read_input;

#[derive(Debug)]
struct Key {
    up: Option<char>,
    right: Option<char>,
    down: Option<char>,
    left: Option<char>,
    value: char,
}

impl Key {
    fn from(input: [[Option<char>; 3]; 3]) -> Self {
        let value = input[1][1].unwrap();
        let up = input[0][1];
        let right = input[1][2];
        let down = input[2][1];
        let left = input[1][0];
        Self {
            value,
            up,
            right,
            down,
            left,
        }
    }
}

#[derive(Debug)]
struct Keypad {
    keys: Vec<Key>,
}

impl Keypad {
    fn get_surrounding_matrix(
        grid: &[Vec<char>],
        row: usize,
        col: usize,
    ) -> [[Option<char>; 3]; 3] {
        let mut matrix = [[None; 3]; 3];

        for di in 0..3 {
            for dj in 0..3 {
                let ni = row as isize + di as isize - 1;
                let nj = col as isize + dj as isize - 1;

                if ni >= 0
                    && (ni as usize) < grid.len()
                    && nj >= 0
                    && (nj as usize) < grid[ni as usize].len()
                {
                    matrix[di][dj] = match grid[ni as usize][nj as usize] {
                        ' ' => None,
                        c => Some(c),
                    };
                }
            }
        }

        matrix
    }

    fn from_str(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let keys = (0..grid.len())
            .flat_map(|i| {
                let value = grid.clone();
                (0..grid[i].len())
                    .map(move |j| {
                        if !value[i][j].is_whitespace() {
                            let matrix = Keypad::get_surrounding_matrix(&value, i, j);
                            Some(Key::from(matrix))
                        } else {
                            None
                        }
                    })
                    .filter(|c| c.is_some())
                    .map(|c| c.unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { keys }
    }

    fn get(&self, key: char) -> &Key {
        self.keys.iter().find(|&k| k.value == key).unwrap()
    }

    fn move_from(&self, from_key: char, movements: &[Move]) -> char {
        if movements.is_empty() {
            return from_key;
        }

        let (movement, rest) = movements.split_first().unwrap();
        let key = self.get(from_key);
        let next_key = match movement {
            Move::Up => key.up,
            Move::Right => key.right,
            Move::Down => key.down,
            Move::Left => key.left,
        };
        if let Some(next_key) = next_key {
            self.move_from(next_key, rest)
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
                let next_digit = self.move_from(code.chars().last().unwrap_or('5'), &movements);
                code.push(next_digit);
                code
            })
    }
}

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

fn main() {
    let keypad_part1 = r#"
123
456
789
    "#;
    let input = read_input("day02.txt");
    let keypad = Keypad::from_str(keypad_part1);
    println!("Part 1: {}", keypad.get_code(input.as_str()));
    let keypad_part2 = r#"
  1
 234
56789
 ABC
  D
    "#;
    let keypad = Keypad::from_str(keypad_part2);
    println!("Part 2: {}", keypad.get_code(input.as_str()));
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn part1() {
        let keypad_input = r#"
123
456
789"#;
        let input = r#"ULL
RRDDD
LURDL
UUUUD"#;
        let keypad = Keypad::from_str(keypad_input);
        assert_eq!(keypad.get_code(input), "1985");
    }

    #[test]
    fn part2() {
        let keypad_input = r#"
  1   
 234
56789
 ABC
  D   "#;
        let keypad = Keypad::from_str(keypad_input);
        let input = r#"ULL
RRDDD
LURDL
UUUUD"#;
        assert_eq!(keypad.get_code(input), "5DB3");
    }
}
