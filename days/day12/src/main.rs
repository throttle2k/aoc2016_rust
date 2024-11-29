use core::panic;
use std::{collections::HashMap, usize};

use common::read_input;

#[derive(Debug)]
enum RegisterOrValue {
    Register(char),
    Value(i32),
}

impl From<&str> for RegisterOrValue {
    fn from(value: &str) -> Self {
        if let Ok(n) = value.parse::<i32>() {
            Self::Value(n)
        } else if let Some(c) = value.chars().nth(0) {
            Self::Register(c)
        } else {
            panic!("Unknown entity {value}")
        }
    }
}

fn str_to_char(s: &str) -> char {
    if s.len() != 1 {
        panic!("String {s} is not a char");
    }
    s.chars().nth(0).unwrap()
}

#[derive(Debug)]
enum Instruction {
    Cpy(RegisterOrValue, char),
    Inc(char),
    Dec(char),
    Jnz(RegisterOrValue, i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        match split.next().unwrap() {
            "cpy" => Self::Cpy(
                split.next().unwrap().into(),
                str_to_char(split.next().unwrap()),
            ),
            "inc" => Self::Inc(str_to_char(split.next().unwrap())),
            "dec" => Self::Dec(str_to_char(split.next().unwrap())),
            "jnz" => Self::Jnz(
                split.next().unwrap().into(),
                split.next().unwrap().parse().unwrap(),
            ),
            s => panic!("Unknown instruction {s}"),
        }
    }
}

struct Computer {
    registers: HashMap<char, i32>,
    instructions: Vec<Instruction>,
    cursor: i32,
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let instructions = value
            .trim()
            .lines()
            .map(|l| l.trim().into())
            .collect::<Vec<Instruction>>();
        let mut registers = HashMap::new();
        registers.insert('a', 0);
        registers.insert('b', 0);
        registers.insert('c', 0);
        registers.insert('d', 0);
        Self {
            registers,
            instructions,
            cursor: 0,
        }
    }
}

impl Computer {
    fn set_register(&mut self, r: char, v: i32) {
        self.registers.entry(r).and_modify(|value| *value = v);
    }

    fn execute(&mut self) {
        while let Some(instruction) = self.instructions.get(self.cursor as usize) {
            match instruction {
                Instruction::Cpy(v, r) => {
                    match v {
                        RegisterOrValue::Register(reg) => {
                            let reg_value = self.registers.get(reg).unwrap().clone();
                            self.registers
                                .entry(*r)
                                .and_modify(|value| *value = reg_value)
                                .or_insert(reg_value)
                        }
                        RegisterOrValue::Value(val) => self
                            .registers
                            .entry(*r)
                            .and_modify(|value| *value = *val)
                            .or_insert(*val),
                    };
                    self.cursor += 1;
                }
                Instruction::Inc(r) => {
                    self.registers
                        .entry(*r)
                        .and_modify(|value| *value += 1)
                        .or_insert(1);
                    self.cursor += 1;
                }
                Instruction::Dec(r) => {
                    self.registers
                        .entry(*r)
                        .and_modify(|value| *value -= 1)
                        .or_insert(-1);
                    self.cursor += 1;
                }
                Instruction::Jnz(v, c) => {
                    let must_jump = match v {
                        RegisterOrValue::Register(reg) => *self.registers.get(reg).unwrap() != 0,
                        RegisterOrValue::Value(val) => *val != 0,
                    };
                    if must_jump {
                        self.cursor += c;
                    } else {
                        self.cursor += 1;
                    }
                }
            };
        }
    }
}

fn main() {
    let input = read_input("day12.txt");
    let mut computer = Computer::from(input.as_str());
    computer.execute();
    println!("Part 1 = {}", *computer.registers.get(&'a').unwrap());
    let mut computer = Computer::from(input.as_str());
    computer.set_register('c', 1);
    computer.execute();
    println!("Part 2 = {}", *computer.registers.get(&'a').unwrap());
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"#;
        let mut computer = Computer::from(input);
        computer.execute();
        assert_eq!(*computer.registers.get(&'a').unwrap(), 42);
    }
}
