use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum Cell {
    Floor,
    Wall,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Floor => ".".to_owned(),
            Cell::Wall => "#".to_owned(),
        }
    }
}

impl Cell {
    fn is_valid(&self) -> bool {
        match self {
            Cell::Floor => true,
            Cell::Wall => false,
        }
    }
}

struct Maze {
    grid: HashMap<(usize, usize), Cell>,
    designer_number: usize,
}

impl Maze {
    fn new(designer_number: usize) -> Self {
        Self {
            grid: HashMap::new(),
            designer_number,
        }
    }

    fn cell_at(&mut self, (x, y): (usize, usize)) -> Cell {
        if let Some(c) = self.grid.get(&(x, y)) {
            c.clone()
        } else {
            let mut value = x * x + 3 * x + 2 * x * y + y + y * y;
            value += self.designer_number;
            let binary = format!("{:b}", value);
            let count_1 = binary.chars().filter(|c| *c == '1').count();
            if count_1 % 2 == 0 {
                self.grid.insert((x, y), Cell::Floor);
                self.grid.get(&(x, y)).unwrap().clone()
            } else {
                self.grid.insert((x, y), Cell::Wall);
                self.grid.get(&(x, y)).unwrap().clone()
            }
        }
    }

    fn steps_to(
        &mut self,
        source_x: usize,
        source_y: usize,
        target_x: usize,
        target_y: usize,
    ) -> Result<Vec<(usize, usize)>, ()> {
        let mut next_steps: HashSet<(usize, usize)> = HashSet::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        next_steps.insert((source_x, source_y));
        let mut path: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut g_score: HashMap<(usize, usize), i32> = HashMap::new();
        g_score.insert((source_x, source_y), 0);
        let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();
        f_score.insert(
            (source_x, source_y),
            distance((source_x, source_y), (target_x, target_y)),
        );
        while !next_steps.is_empty() {
            let current = next_steps
                .iter()
                .cloned()
                .min_by_key(|&p| f_score.get(&p).unwrap())
                .unwrap();
            if current == (target_x, target_y) {
                return Ok(rebuild_path((source_x, source_y), &path, current, vec![]));
            }
            next_steps.remove(&current);
            visited.insert(current.clone());
            neighbors_of(current)
                .iter()
                .filter(|&c| self.cell_at(*c).is_valid())
                .for_each(|neighbor| {
                    if !visited.contains(neighbor) {
                        let trial_score = g_score.get(&current).unwrap() + 1;
                        if trial_score < *g_score.get(&neighbor).unwrap_or(&i32::max_value()) {
                            path.insert(*neighbor, current.clone());
                            g_score.insert(*neighbor, trial_score);
                            f_score.insert(
                                *neighbor,
                                trial_score as usize + distance(*neighbor, (target_x, target_y)),
                            );
                            next_steps.insert(neighbor.clone());
                        }
                    }
                })
        }
        Err(())
    }
}

fn neighbors_of((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    neighbors.push((x + 1, y));
    neighbors.push((x, y + 1));
    neighbors
}

fn distance((source_x, source_y): (usize, usize), (target_x, target_y): (usize, usize)) -> usize {
    let dx = if source_x > target_x {
        source_x - target_x
    } else {
        target_x - source_x
    };
    let dy = if source_y > target_y {
        source_y - target_y
    } else {
        target_y - source_y
    };
    dx + dy
}

fn rebuild_path(
    (source_x, source_y): (usize, usize),
    path: &HashMap<(usize, usize), (usize, usize)>,
    current: (usize, usize),
    mut full_path: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    if current == (source_x, source_y) {
        full_path.reverse();
        return full_path;
    }
    full_path.push(current);
    rebuild_path(
        (source_x, source_y),
        path,
        *path.get(&current).unwrap(),
        full_path,
    )
}

fn main() {
    let mut maze = Maze::new(1364);
    let steps = maze.steps_to(1, 1, 31, 39).unwrap();
    println!("Part 1 = {}", steps.len());
    // let grid = (0..45)
    //     .map(|col| {
    //         let mut row = (0..45)
    //             .map(|row| {
    //                 if steps.contains(&(row, col)) {
    //                     "O".to_owned()
    //                 } else {
    //                     maze.cell_at((row, col)).to_string()
    //                 }
    //             })
    //             .collect::<String>();
    //         row.push('\n');
    //         row
    //     })
    //     .collect::<String>();
    // println!("{grid}");
    let targets = (0..51)
        .flat_map(|col| {
            (0..51)
                .filter_map(move |row| {
                    if distance((1, 1), (row.clone(), col.clone())) < 50 {
                        Some((row.clone(), col.clone()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let paths_of_50 = targets
        .iter()
        .filter_map(|(tx, ty)| maze.steps_to(1, 1, *tx, *ty).ok())
        .filter(|steps| steps.len() <= 50)
        .count();
    println!("Part 2 = {}", paths_of_50);
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn maze_mapping() {
        let mut maze = Maze::new(10);
        let grid = (0..7)
            .map(|col| {
                let mut row = (0..10)
                    .map(|row| maze.cell_at((row, col)).to_string())
                    .collect::<String>();
                row.push('\n');
                row
            })
            .collect::<String>();
        assert_eq!(
            grid.trim(),
            r#".#.####.##
..#..#...#
#....##...
###.#.###.
.##..#..#.
..##....#.
#...##.###"#
        );
    }

    #[test]
    fn part1() {
        let mut maze = Maze::new(10);
        let steps = maze.steps_to(1, 1, 7, 4).unwrap();
        assert_eq!(steps.len(), 11);
        let grid = (0..7)
            .map(|col| {
                let mut row = (0..10)
                    .map(|row| {
                        if steps.contains(&(row, col)) {
                            "O".to_owned()
                        } else {
                            maze.cell_at((row, col)).to_string()
                        }
                    })
                    .collect::<String>();
                row.push('\n');
                row
            })
            .collect::<String>();
        println!("{grid}");
    }
}
