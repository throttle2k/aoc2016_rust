use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use common::read_input;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Material {
    Hydrogen,
    Lithium,
    Polonium,
    Thulium,
    Promethium,
    Ruthenium,
    Cobalt,
    Elerium,
    Dilithium,
}

impl ToString for Material {
    fn to_string(&self) -> String {
        match self {
            Material::Hydrogen => "H".to_string(),
            Material::Lithium => "Li".to_string(),
            Material::Polonium => "Po".to_string(),
            Material::Thulium => "Th".to_string(),
            Material::Promethium => "Pr".to_string(),
            Material::Ruthenium => "R".to_string(),
            Material::Cobalt => "Co".to_string(),
            Material::Elerium => "El".to_string(),
            Material::Dilithium => "Di".to_string(),
        }
    }
}

impl From<&str> for Material {
    fn from(value: &str) -> Self {
        match value {
            "hydrogen" | "hydrogen-compatible" => Self::Hydrogen,
            "lithium" | "lithium-compatible" => Self::Lithium,
            "polonium" | "polonium-compatible" => Self::Polonium,
            "thulium" | "thulium-compatible" => Self::Thulium,
            "promethium" | "promethium-compatible" => Self::Promethium,
            "ruthenium" | "ruthenium-compatible" => Self::Ruthenium,
            "cobalt" | "cobalt-compatible" => Self::Cobalt,
            "elerium" | "elerium-compatible" => Self::Elerium,
            "dilithium" | "dilithium-compatible" => Self::Dilithium,
            m => panic!("Unknown material {m}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Item {
    Generator(Material),
    Microchip(Material),
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match self {
            Item::Generator(m) => format!("G{}", m.to_string()),
            Item::Microchip(m) => format!("M{}", m.to_string()),
        }
    }
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        let (material, item) = value
            .trim()
            .strip_prefix("a ")
            .unwrap()
            .split_once(' ')
            .unwrap();
        match item {
            "generator" => Self::Generator(material.into()),
            "microchip" => Self::Microchip(material.into()),
            i => panic!("Unknown item {i}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Floor {
    microchips: HashSet<Item>,
    generators: HashSet<Item>,
}

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        self.generators.len() == other.generators.len()
            && self.microchips.len() == other.microchips.len()
    }
}

impl Hash for Floor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.microchips.len().hash(state);
        self.generators.len().hash(state);
    }
}

impl ToString for Floor {
    fn to_string(&self) -> String {
        let mut string = self.generators.iter().fold(String::new(), |mut s, g| {
            s.push_str(&g.to_string());
            s
        });
        string = self.microchips.iter().fold(string, |mut s, m| {
            s.push_str(&m.to_string());
            s
        });
        string
    }
}

impl From<Vec<Item>> for Floor {
    fn from(value: Vec<Item>) -> Self {
        let (microchips, generators) = value.iter().cloned().fold(
            (HashSet::new(), HashSet::new()),
            |(mut microchips, mut generators), item| {
                match item {
                    g @ Item::Generator(_) => generators.insert(g),
                    m @ Item::Microchip(_) => microchips.insert(m),
                };
                (microchips, generators)
            },
        );
        Self {
            microchips,
            generators,
        }
    }
}

impl Floor {
    fn add_items(&mut self, items: Vec<Item>) {
        items.iter().cloned().for_each(|item| {
            match item {
                g @ Item::Generator(_) => self.generators.insert(g),
                m @ Item::Microchip(_) => self.microchips.insert(m),
            };
        });
    }

    fn remove_items(&mut self, items: &Vec<Item>) {
        items.iter().for_each(|item| {
            match item {
                g @ Item::Generator(_) => self.generators.remove(g),
                m @ Item::Microchip(_) => self.microchips.remove(m),
            };
        });
    }

    fn all_items(&self) -> Vec<Item> {
        let mut items = self
            .microchips
            .iter()
            .cloned()
            .fold(vec![], |mut items, item| {
                items.push(item);
                items
            });
        items = self
            .generators
            .iter()
            .cloned()
            .fold(items, |mut items, item| {
                items.push(item);
                items
            });
        items
    }

    fn is_valid(&self) -> bool {
        if self.microchips.is_empty() || self.generators.is_empty() {
            return true;
        }
        self.microchips.iter().all(|m| match m {
            Item::Microchip(m) => self.generators.contains(&Item::Generator(m.clone())),
            i => panic!("Wrong item in microchips {:?}", i),
        })
    }
}

#[derive(PartialEq, Hash, Clone, Debug)]
struct FacilityStatus {
    elevator_level: usize,
    floors: Vec<String>,
}

impl From<&Facility> for FacilityStatus {
    fn from(value: &Facility) -> Self {
        let elevator_level = value.elevator_level;
        let mut floors = value
            .floors
            .iter()
            .map(|(n, f)| format!("{n}{}", f.to_string()))
            .collect::<Vec<_>>();
        floors.sort();
        Self {
            elevator_level,
            floors,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Facility {
    num_floors: usize,
    floors: HashMap<usize, Floor>,
    elevator_level: usize,
}

impl From<&str> for Facility {
    fn from(value: &str) -> Self {
        let num_floors = value.lines().count();
        let floors = value.lines().fold(HashMap::new(), |mut floors, line| {
            let line = line.replace(", and", ",");
            let line = line.replace(" and", ",");
            let line = line.trim().strip_suffix(".").unwrap();
            let (_, line) = line.split_once(' ').unwrap();
            let (floor_num, line) = line.split_once(' ').unwrap();
            let floor_num = match floor_num {
                "first" => 0,
                "second" => 1,
                "third" => 2,
                "fourth" => 3,
                f => panic!("Unknown floor {f}"),
            };
            let (_, line) = line.split_once(' ').unwrap();
            let (_, line) = line.split_once(' ').unwrap();
            let items: Vec<Item> = line
                .split(',')
                .filter_map(|item| match item {
                    "nothing relevant" => None,
                    i => Some(i.into()),
                })
                .collect();
            floors.entry(floor_num).or_insert(items.into());
            floors
        });
        Self {
            num_floors,
            floors,
            elevator_level: 0,
        }
    }
}

fn all_combinations_of_size(v: &Floor, n: usize) -> Vec<Vec<Item>> {
    fn combine_rec(
        start: usize,
        items: &[Item],
        n: usize,
        current: &mut Vec<Item>,
        result: &mut Vec<Vec<Item>>,
    ) {
        if current.len() == n {
            current.sort();
            result.push(current.clone());
            return;
        }
        (start..items.len()).for_each(|i| {
            current.push(items[i].clone());
            combine_rec(i + 1, items, n, current, result);
            current.pop();
        })
    }

    let mut result = vec![];
    let mut current = vec![];
    let items = v.all_items();
    combine_rec(0, &items, n, &mut current, &mut result);
    result.sort();
    result.dedup();
    result
}

impl Facility {
    fn is_final(&self) -> bool {
        (0..self.num_floors - 1)
            .map(|n| self.floors.get(&n).unwrap())
            .all(|f| f.microchips.is_empty() && f.generators.is_empty())
    }

    fn add_item(&mut self, item: Item, floor: usize) {
        self.floors.get_mut(&floor).unwrap().add_items(vec![item]);
    }

    fn next_states(&self) -> Vec<Self> {
        let mut combinations =
            all_combinations_of_size(self.floors.get(&self.elevator_level).unwrap(), 2);
        combinations.extend(all_combinations_of_size(
            self.floors.get(&self.elevator_level).unwrap(),
            1,
        ));
        let mut next_states = vec![];
        combinations.iter().for_each(|comb| {
            let mut current_floor = self.floors.get(&self.elevator_level).unwrap().clone();
            current_floor.remove_items(comb);
            if self.elevator_level < self.num_floors - 1 {
                let mut next_floor = self.floors.get(&(self.elevator_level + 1)).unwrap().clone();
                next_floor.add_items(comb.clone());
                if next_floor.is_valid() {
                    let mut floors = self.floors.clone();
                    *floors.get_mut(&self.elevator_level).unwrap() = current_floor.clone();
                    *floors.get_mut(&(self.elevator_level + 1)).unwrap() = next_floor.clone();
                    next_states.push(Facility {
                        num_floors: self.num_floors,
                        floors,
                        elevator_level: self.elevator_level + 1,
                    });
                }
            }
            if self.elevator_level > 0 && self.elevator_level <= self.num_floors - 1 {
                let mut next_floor = self.floors.get(&(self.elevator_level - 1)).unwrap().clone();
                next_floor.add_items(comb.clone());
                if next_floor.is_valid() {
                    let mut floors = self.floors.clone();
                    *floors.get_mut(&self.elevator_level).unwrap() = current_floor.clone();
                    *floors.get_mut(&(self.elevator_level - 1)).unwrap() = next_floor.clone();
                    next_states.push(Facility {
                        num_floors: self.num_floors,
                        floors,
                        elevator_level: self.elevator_level - 1,
                    });
                }
            }
        });
        next_states
    }

    fn score(&self) -> usize {
        self.floors
            .iter()
            .map(|(n, floor)| floor.all_items().len() * 16usize.pow(*n as u32))
            .sum()
    }

    fn count_steps_a_star(&self) -> usize {
        let mut visited = vec![];
        let mut next_states = vec![(self.clone(), 0)];
        let mut max = 0;

        while !next_states.is_empty() {
            let (current_state, count) = next_states.remove(0);
            if current_state.is_final() {
                return count;
            }
            if current_state.score() > max {
                max = current_state.score();
            }
            let mut neighbors = current_state.next_states();
            neighbors.sort_by(|s1, s2| s2.score().cmp(&s1.score()));
            neighbors
                .iter()
                .filter(|n| max - n.score().clamp(0, max) < 8192)
                .for_each(|neighbor| {
                    if !visited.contains(neighbor) {
                        next_states.push((neighbor.clone(), count + 1));
                        visited.push(neighbor.clone());
                    }
                })
        }
        0
    }
}

fn main() {
    let input = read_input("day11.txt");
    let mut facility = Facility::from(input.as_str());
    println!("Part 1 = {}", facility.count_steps_a_star());
    facility.add_item(Item::Generator(Material::Elerium), 0);
    facility.add_item(Item::Microchip(Material::Elerium), 0);
    facility.add_item(Item::Generator(Material::Dilithium), 0);
    facility.add_item(Item::Microchip(Material::Dilithium), 0);
    println!("Part 2 = {}", facility.count_steps_a_star());
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn only_a_microchip() {
        let input = r#"The first floor contains a hydrogen-compatible microchip.
The second floor contains nothing relevant.
The third floor contains nothing relevant.
The fourth floor contains nothing relevant."#;
        let facility = Facility::from(input);
        assert_eq!(facility.count_steps_a_star(), 3);
    }

    #[test]
    fn only_a_generator() {
        let input = r#"The first floor contains a hydrogen generator.
The second floor contains nothing relevant.
The third floor contains nothing relevant.
The fourth floor contains nothing relevant."#;
        let facility = Facility::from(input);
        assert_eq!(facility.count_steps_a_star(), 3);
    }

    #[test]
    fn a_microchip_and_a_generator() {
        let input = r#"The first floor contains a hydrogen generator.
The second floor contains a hydrogen-compatible microchip.
The third floor contains nothing relevant.
The fourth floor contains nothing relevant."#;
        let facility = Facility::from(input);
        assert_eq!(facility.count_steps_a_star(), 3);
    }

    #[test]
    fn part1() {
        let input = r#"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
    The second floor contains a hydrogen generator.
    The third floor contains a lithium generator.
    The fourth floor contains nothing relevant."#;
        let facility = Facility::from(input);
        assert_eq!(facility.count_steps_a_star(), 11);
    }
}
