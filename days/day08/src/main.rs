use std::usize;

use common::read_input;

enum Instruction {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value.starts_with("rect") {
            let input = value.strip_prefix("rect ").unwrap();
            let (cols, rows) = input.split_once('x').unwrap();
            Self::Rect(
                cols.parse::<usize>().unwrap(),
                rows.parse::<usize>().unwrap(),
            )
        } else if value.starts_with("rotate column") {
            let input = value.strip_prefix("rotate column x=").unwrap();
            let (column, qty) = input.split_once(" by ").unwrap();
            Self::RotateColumn(
                column.parse::<usize>().unwrap(),
                qty.parse::<usize>().unwrap(),
            )
        } else if value.starts_with("rotate row") {
            let input = value.strip_prefix("rotate row y=").unwrap();
            let (row, qty) = input.split_once(" by ").unwrap();
            Self::RotateRow(row.parse::<usize>().unwrap(), qty.parse::<usize>().unwrap())
        } else {
            panic!("Unknown instruction {value}")
        }
    }
}

struct SmallDisplay {
    rows: usize,
    cols: usize,
    pixels: Vec<char>,
}

impl SmallDisplay {
    fn new(rows: usize, cols: usize) -> Self {
        SmallDisplay {
            rows,
            cols,
            pixels: vec!['.'; rows * cols],
        }
    }

    fn count_lit_pixels(&self) -> usize {
        self.pixels.iter().filter(|&c| *c == '#').count()
    }

    fn rotate_sequence(sequence: Vec<char>, n: usize) -> Vec<char> {
        if n == 0 {
            return sequence;
        }
        let (last, rest) = sequence.split_last().unwrap();
        let mut new_sequence = Vec::with_capacity(sequence.len());
        new_sequence.push(last.clone());
        new_sequence.extend_from_slice(rest);
        Self::rotate_sequence(new_sequence, n - 1)
    }

    fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rect(cols, rows) => (0..rows).for_each(|r| {
                (0..cols).for_each(|c| {
                    let idx = r * self.cols + c;
                    *self.pixels.get_mut(idx).unwrap() = '#';
                })
            }),
            Instruction::RotateColumn(col, qty) => {
                let qty = qty % self.rows;
                let col_pixels = (0..self.rows)
                    .map(|r| {
                        let idx = r * self.cols + col;
                        self.pixels.get(idx).unwrap().clone()
                    })
                    .collect::<Vec<_>>();
                let new_col_pixels = Self::rotate_sequence(col_pixels, qty);
                new_col_pixels.iter().enumerate().for_each(|(r, char)| {
                    let idx = r * self.cols + col;
                    *self.pixels.get_mut(idx).unwrap() = *char;
                });
            }
            Instruction::RotateRow(row, qty) => {
                let qty = qty % self.cols;
                let row_pixels = (0..self.cols)
                    .map(|c| {
                        let idx = row * self.cols + c;
                        self.pixels.get(idx).unwrap().clone()
                    })
                    .collect::<Vec<_>>();
                let new_row_pixels = Self::rotate_sequence(row_pixels, qty);
                new_row_pixels.iter().enumerate().for_each(|(c, char)| {
                    let idx = row * self.cols + c;
                    *self.pixels.get_mut(idx).unwrap() = *char;
                });
            }
        }
    }
}

impl ToString for SmallDisplay {
    fn to_string(&self) -> String {
        self.pixels
            .chunks(self.cols)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn main() {
    let input = read_input("day08.txt");
    let mut display = SmallDisplay::new(6, 50);
    let instructions = input
        .trim()
        .lines()
        .map(|l| Instruction::from(l.trim()))
        .collect::<Vec<_>>();
    instructions.into_iter().for_each(|i| display.apply(i));
    println!("Part 1 = {}", display.count_lit_pixels());
    println!("Part 2 = \n{}", display.to_string());
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn part1() {
        let mut display = SmallDisplay::new(3, 7);
        assert_eq!(
            display.to_string(),
            r#".......
.......
......."#
        );
        display.apply(Instruction::from("rect 3x2"));
        assert_eq!(
            display.to_string(),
            r#"###....
###....
......."#
        );
        display.apply(Instruction::from("rotate column x=1 by 1"));
        assert_eq!(
            display.to_string(),
            r#"#.#....
###....
.#....."#
        );
        display.apply(Instruction::from("rotate row y=0 by 4"));
        assert_eq!(
            display.to_string(),
            r#"....#.#
###....
.#....."#
        );
        display.apply(Instruction::from("rotate column x=1 by 1"));
        assert_eq!(
            display.to_string(),
            r#".#..#.#
#.#....
.#....."#
        );
    }
}
