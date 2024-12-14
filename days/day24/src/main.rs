use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
    ops::Deref,
};

use common::read_input;

#[derive(Debug, PartialEq)]
enum Tile {
    Checkpoint(usize),
    Floor,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Floor,
            '#' => Self::Wall,
            c if c.is_digit(10) => Self::Checkpoint(c.to_digit(10).unwrap() as usize),
            c => panic!("Unknown tile {c}"),
        }
    }
}

impl Tile {
    fn is_walkable(&self) -> bool {
        match self {
            Tile::Wall => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
    checkpoints: Vec<usize>,
    cost: HashMap<(usize, usize), usize>,
}

impl Deref for Maze {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let tiles = value
            .trim()
            .lines()
            .map(|l| l.trim().chars().map(|c| c.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = tiles.len();
        let cols = tiles.iter().nth(0).unwrap().len();
        let checkpoints = tiles
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter_map(|t| match t {
                        Tile::Checkpoint(p) => Some(*p),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let mut maze = Self {
            tiles,
            rows,
            cols,
            checkpoints,
            cost: HashMap::new(),
        };
        maze.prepare_distance_matrix();
        maze
    }
}

impl Maze {
    fn cp_position(&self, cp: usize) -> (usize, usize) {
        *(0..self.rows)
            .flat_map(|row| (0..self.cols).map(|col| (col, row)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .iter()
            .find(|(col, row)| self[*row][*col] == Tile::Checkpoint(cp))
            .unwrap()
    }

    fn neighbors(&self, (position_x, position_y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut deltas: Vec<(isize, isize)> = vec![];
        if self[position_y][position_x - 1].is_walkable() {
            deltas.push((-1, 0));
        };
        if self[position_y][position_x + 1].is_walkable() {
            deltas.push((1, 0));
        };
        if self[position_y - 1][position_x].is_walkable() {
            deltas.push((0, -1));
        };
        if self[position_y + 1][position_x].is_walkable() {
            deltas.push((0, 1));
        };
        deltas
            .iter()
            .map(|(delta_x, delta_y)| {
                (
                    (position_x as isize + delta_x) as usize,
                    (position_y as isize + delta_y) as usize,
                )
            })
            .collect()
    }

    fn min_steps_from_to(&self, cp_start: usize, cp_end: usize) -> usize {
        let start = self.cp_position(cp_start);
        let end = self.cp_position(cp_end);
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        let mut visited = vec![start];
        while let Some((current, depth)) = queue.pop_front() {
            if current == end {
                return depth;
            }
            self.neighbors(current).iter().for_each(|neighbor| {
                if !visited.contains(neighbor) {
                    visited.push(*neighbor);
                    queue.push_back((*neighbor, depth + 1));
                }
            });
        }
        unreachable!("Target cannot be reached");
    }

    fn prepare_distance_matrix(&mut self) {
        let mut distance_matrix: HashMap<(usize, usize), usize> = HashMap::new();
        self.checkpoints
            .iter()
            .enumerate()
            .for_each(|(idx_from, cp_start)| {
                self.checkpoints[idx_from..].iter().for_each(|cp_end| {
                    let distance = self.min_steps_from_to(*cp_start, *cp_end);
                    distance_matrix.insert((*cp_start, *cp_end), distance);
                    distance_matrix.insert((*cp_end, *cp_start), distance);
                })
            });
        self.cost = distance_matrix;
    }

    fn total_cost(
        &self,
        mask: usize,
        current: usize,
        n: usize,
        memo: &mut Vec<Vec<Option<usize>>>,
        and_return: bool,
    ) -> usize {
        if mask == (1 << n) - 1 {
            if and_return {
                return *self.cost.get(&(current, 0)).unwrap();
            } else {
                return 0;
            }
        }

        if let Some(result) = memo[current][mask] {
            return result;
        }

        let mut result = usize::max_value();
        (0..n).for_each(|i| {
            if mask & (1 << i) == 0 {
                result = min(
                    result,
                    self.cost.get(&(current, i)).unwrap()
                        + self.total_cost(mask | (1 << i), i, n, memo, and_return),
                );
            }
        });
        memo[current][mask] = Some(result);
        result
    }

    fn tsp(&self, and_return: bool) -> usize {
        let n = self.checkpoints.iter().max().unwrap() + 1;
        let mut memo: Vec<Vec<Option<usize>>> = vec![vec![None; 1 << n]; n];
        self.total_cost(1, 0, n, &mut memo, and_return)
    }
}

fn main() {
    let input = read_input("day24.txt");
    let maze = Maze::from(input.as_str());
    println!("Part 1 = {}", maze.tsp(false));
    println!("Part 2 = {}", maze.tsp(true));
}

#[cfg(test)]
mod day24_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        from = { 0, 4, 1, 2 },
        to = { 4, 1, 2, 3 },
        expected = { 2, 4, 6, 2 }
    )]
    fn test_min_steps_from_to(from: usize, to: usize, expected: usize) {
        let input = r#"###########
#0.1.....2#
#.#######.#
#4.......3#
###########"#;
        let maze = Maze::from(input);
        assert_eq!(maze.min_steps_from_to(from, to), expected);
    }

    #[test]
    fn part1() {
        let input = r#"###########
#0.1.....2#
#.#######.#
#4.......3#
###########"#;
        let maze = Maze::from(input);
        assert_eq!(maze.tsp(false), 14);
    }
}
