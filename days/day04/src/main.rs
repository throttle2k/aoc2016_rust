use std::{char, collections::HashMap};

use common::read_input;

struct Room {
    encrypted_name: String,
    id: u32,
    checksum: String,
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        let value = value.chars().rev().collect::<String>();
        let (rest, encrypted_name) = value.split_once('-').unwrap();
        let encrypted_name = encrypted_name.chars().rev().collect::<String>();
        let rest = rest.chars().rev().collect::<String>();
        let (id, checksum) = rest.split_at(3);
        let id = id.parse::<u32>().unwrap();
        let checksum = checksum.chars().collect::<String>();
        let checksum = checksum
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .to_string();
        Self {
            encrypted_name,
            id,
            checksum,
        }
    }
}

fn shift_char(c: char, n: u32) -> char {
    if n == 0 {
        return c;
    }

    if c == '-' {
        return ' ';
    }

    let new_c = match c {
        'z' => 'a',
        c => char::from_u32(c as u32 + 1).unwrap(),
    };
    shift_char(new_c, n - 1)
}

impl Room {
    fn calculate_checksum(&self) -> String {
        let count_map =
            self.encrypted_name
                .chars()
                .fold(HashMap::<char, usize>::new(), |mut count_map, c| match c {
                    '-' => count_map,
                    c => {
                        count_map.entry(c).and_modify(|i| *i += 1).or_insert(1);
                        count_map
                    }
                });
        let count_map =
            count_map
                .iter()
                .fold(HashMap::<usize, String>::new(), |mut count_map, (k, v)| {
                    count_map
                        .entry(*v)
                        .and_modify(|s| {
                            s.push(*k);
                            let mut new_s = s.clone().chars().collect::<Vec<_>>();
                            new_s.sort();
                            *s = new_s.iter().collect::<String>();
                        })
                        .or_insert(format!("{k}"));
                    count_map
                });
        let mut frequencies = count_map.keys().collect::<Vec<_>>();

        frequencies.sort_by(|a, b| b.cmp(&a));
        let s = frequencies.iter().take(5).fold(String::new(), |mut s, f| {
            let other_s = count_map.get(f).unwrap();
            s.push_str(other_s);
            s
        });
        s.chars().take(5).collect()
    }

    fn is_real(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    fn decode_name(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| shift_char(c, self.id))
            .collect::<String>()
    }
}

fn main() {
    let input = read_input("day04.txt");
    let id_sum = input
        .lines()
        .filter_map(|l| {
            let room = Room::from(l);
            if room.is_real() {
                Some(room.id)
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("Part 1 = {}", id_sum);
    let room = input
        .lines()
        .map(|l| Room::from(l))
        .filter(|r| r.is_real())
        .find(|r| r.decode_name() == "northpole object storage")
        .unwrap();
    println!("Part 2 = {}", room.id);
}

#[cfg(test)]
mod day04_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = {"aaaaa-bbb-z-y-x-123[abxyz]", "a-b-c-d-e-f-g-h-987[abcde]", "not-a-real-room-404[oarel]", "totally-real-room-200[decoy]"},
        expected = {true, true, true, false}
    )]
    fn part1_check_real(input: &str, expected: bool) {
        let room = Room::from(input);
        assert_eq!(room.is_real(), expected);
    }

    #[test]
    fn part1_check_real() {
        let input = r#"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"#;
        let rooms = input
            .lines()
            .filter_map(|l| {
                let room = Room::from(l);
                if room.is_real() {
                    Some(room)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(rooms.iter().map(|r| r.id).sum::<u32>(), 1514);
    }

    #[test]
    fn part2() {
        let input = "qzmt-zixmtkozy-ivhz-343[zimtq]";
        let room = Room::from(input);
        assert_eq!(room.decode_name(), "very encrypted name".to_string());
    }
}
