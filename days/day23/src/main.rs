use std::collections::HashMap;

use common::read_input;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
enum Instruction {
    Cpy(RegisterOrValue, RegisterOrValue),
    Inc(char),
    Dec(char),
    Jnz(RegisterOrValue, RegisterOrValue),
    Tgl(char),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        match split.next().unwrap() {
            "cpy" => Self::Cpy(split.next().unwrap().into(), split.next().unwrap().into()),
            "inc" => Self::Inc(str_to_char(split.next().unwrap())),
            "dec" => Self::Dec(str_to_char(split.next().unwrap())),
            "jnz" => Self::Jnz(split.next().unwrap().into(), split.next().unwrap().into()),
            "tgl" => Self::Tgl(str_to_char(split.next().unwrap())),
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

fn process_multiply(instructions: &[Instruction]) -> Option<(char, char, char, char)> {
    let cpy =
        if let Instruction::Cpy(RegisterOrValue::Register(r1), RegisterOrValue::Register(r2)) =
            instructions[0]
        {
            (r1, r2)
        } else {
            return None;
        };
    let inc = if let Instruction::Inc(r) = instructions[1] {
        r
    } else {
        return None;
    };
    let dec1 = if let Instruction::Dec(r) = instructions[2] {
        r
    } else {
        return None;
    };
    let jnz1 = if let Instruction::Jnz(RegisterOrValue::Register(r), _) = instructions[3] {
        r
    } else {
        return None;
    };
    let dec2 = if let Instruction::Dec(r) = instructions[4] {
        r
    } else {
        return None;
    };
    let jnz2 = if let Instruction::Jnz(RegisterOrValue::Register(r), _) = instructions[5] {
        r
    } else {
        return None;
    };
    if !(cpy.1 == dec1 && cpy.1 == jnz1 && dec2 == jnz2) {
        return None;
    };
    Some((cpy.0, dec2, cpy.1, inc))
}

impl Computer {
    fn set_register(&mut self, r: char, v: i32) {
        self.registers.entry(r).and_modify(|value| *value = v);
    }

    fn execute(&mut self) {
        while let Some(instruction) = self.instructions.get(self.cursor as usize) {
            let process_multiply = if self.cursor as usize + 6 < self.instructions.len() {
                process_multiply(&self.instructions[self.cursor as usize..self.cursor as usize + 6])
            } else {
                None
            };
            if let Some((r1, r2, r3, rd)) = process_multiply {
                let mult1 = self.registers.get(&r1).unwrap().clone();
                let mult2 = self.registers.get(&r2).unwrap().clone();
                self.registers
                    .entry(rd)
                    .and_modify(|value| *value += mult1 * mult2);
                self.registers.entry(r2).and_modify(|value| *value = 0);
                self.registers.entry(r3).and_modify(|value| *value = 0);
                self.cursor += 6;
            } else {
                match instruction {
                    Instruction::Cpy(v, r) => {
                        if let RegisterOrValue::Register(r) = &r {
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
                        }
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
                    Instruction::Jnz(v, c) => match c {
                        RegisterOrValue::Value(c) => {
                            let must_jump = match v {
                                RegisterOrValue::Register(reg) => {
                                    *self.registers.get(reg).unwrap() != 0
                                }
                                RegisterOrValue::Value(val) => *val != 0,
                            };
                            if must_jump {
                                self.cursor += c;
                            } else {
                                self.cursor += 1;
                            }
                        }
                        RegisterOrValue::Register(r) => {
                            let must_jump = match v {
                                RegisterOrValue::Register(reg) => {
                                    *self.registers.get(reg).unwrap() != 0
                                }
                                RegisterOrValue::Value(val) => *val != 0,
                            };
                            if must_jump {
                                self.cursor += self.registers.get(r).unwrap();
                            } else {
                                self.cursor += 1;
                            }
                        }
                    },
                    Instruction::Tgl(r) => {
                        if let Some(pos) = self.registers.get(r) {
                            let pos = pos + self.cursor;
                            if pos < self.instructions.len() as i32 {
                                let new_instruction = if let Some(instruction) =
                                    self.instructions.get(pos as usize)
                                {
                                    match instruction {
                                        Instruction::Inc(r) => Some(Instruction::Dec(*r)),
                                        Instruction::Dec(r) => Some(Instruction::Inc(*r)),
                                        Instruction::Cpy(rov, c) => {
                                            Some(Instruction::Jnz(rov.clone(), c.clone()))
                                        }
                                        Instruction::Jnz(rov, c) => {
                                            Some(Instruction::Cpy(rov.clone(), c.clone()))
                                        }
                                        Instruction::Tgl(r) => Some(Instruction::Inc(*r)),
                                    }
                                } else {
                                    None
                                };
                                if let Some(instruction) = new_instruction {
                                    self.instructions.remove(pos as usize);
                                    self.instructions.insert(pos as usize, instruction);
                                }
                            }
                        }
                        self.cursor += 1;
                    }
                }
            };
        }
    }
}

fn main() {
    let input = read_input("day23.txt");
    let mut computer = Computer::from(input.as_str());
    computer.set_register('a', 7);
    computer.execute();
    println!("Part 1 = {}", *computer.registers.get(&'a').unwrap());
    let mut computer = Computer::from(input.as_str());
    computer.set_register('a', 12);
    computer.execute();
    println!("Part 2 = {}", *computer.registers.get(&'a').unwrap());
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a"#;
        let mut computer = Computer::from(input);
        computer.execute();
        assert_eq!(*computer.registers.get(&'a').unwrap(), 3);
    }
}
