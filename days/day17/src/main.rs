#[derive(Debug, PartialEq)]
enum Door {
    Open,
    Locked,
}

fn get_doors(path: &str) -> Vec<Door> {
    let digest = md5::compute(path);
    format!("{:x}", digest)
        .chars()
        .take(4)
        .map(|c| match c {
            'b' | 'c' | 'd' | 'e' | 'f' => Door::Open,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' => Door::Locked,
            char => panic!("Unexpected character {char}"),
        })
        .collect()
}

#[derive(Debug)]
struct Cell {
    current_position: (usize, usize),
    doors: Vec<Door>,
    path: String,
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        let (pos_x, pos_y) =
            value
                .chars()
                .skip_while(|c| c.is_lowercase())
                .fold((0, 0), |(pos_x, pos_y), c| {
                    let (dx, dy) = match c {
                        'U' => (0, -1),
                        'D' => (0, 1),
                        'L' => (-1, 0),
                        'R' => (1, 0),
                        c => panic!("Unexpected character {c}"),
                    };
                    (pos_x + dx, pos_y + dy)
                });
        let doors = get_doors(value);
        Self {
            current_position: (pos_x as usize, pos_y as usize),
            doors,
            path: value.to_owned(),
        }
    }
}

impl Cell {
    fn movement(&self, m: char) -> Self {
        let mut next_path = self.path.clone();
        next_path.push(m);
        Self::from(next_path.as_str())
    }

    fn get_valid_movements(&self) -> Vec<char> {
        self.doors
            .iter()
            .enumerate()
            .filter_map(|(direction, door)| match direction {
                0 => {
                    if self.current_position.1 > 0 {
                        Some(('U', door))
                    } else {
                        None
                    }
                }
                1 => {
                    if self.current_position.1 < 3 {
                        Some(('D', door))
                    } else {
                        None
                    }
                }
                2 => {
                    if self.current_position.0 > 0 {
                        Some(('L', door))
                    } else {
                        None
                    }
                }
                3 => {
                    if self.current_position.0 < 3 {
                        Some(('R', door))
                    } else {
                        None
                    }
                }
                c => panic!("Unexpected character {c}"),
            })
            .filter_map(|(c, door)| match door {
                Door::Open => Some(c),
                Door::Locked => None,
            })
            .collect()
    }
}

fn find_shortest_path(input: &str) -> String {
    let mut queue = vec![];
    let start = Cell::from(input);
    queue.push(start);
    while !queue.is_empty() {
        let cell = queue.remove(0);
        // println!("{:?}", cell);
        if cell.current_position == (3, 3) {
            let result = cell.path.chars().skip_while(|c| c.is_lowercase()).collect();
            return result;
        };
        cell.get_valid_movements().iter().for_each(|c| {
            queue.push(cell.movement(*c));
        });
    }
    String::new()
}

fn main() {
    println!("Part 1 = {}", find_shortest_path("dmypynyp"));
}

#[cfg(test)]
mod day17_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "hijkl", "hijklD", "hijklDR", "hijklDU", "hijklDUR" },
        expected = { vec![Door::Open, Door::Open, Door::Open, Door::Locked], vec![Door::Open, Door::Locked, Door::Open, Door::Open], vec![Door::Locked, Door::Locked, Door::Locked, Door::Locked], vec![Door::Locked, Door::Locked, Door::Locked, Door::Open], vec![Door::Locked, Door::Locked, Door::Locked, Door::Locked] }
    )]
    fn test_get_doors(input: &str, expected: Vec<Door>) {
        assert_eq!(get_doors(input), expected);
    }

    #[parameterized(
        input = { "ihgpwlah", "kglvqrro", "ulqzkmiv" },
        expected = { "DDRRRD", "DDUDRLRRUDRD", "DRURDRUDDLLDLUURRDULRLDUUDDDRR" }
    )]
    fn part1(input: &str, expected: &str) {
        assert_eq!(find_shortest_path(input), expected);
    }
}
