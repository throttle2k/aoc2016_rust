use md5;
use std::collections::HashMap;

fn find_match(
    n: u32,
    input: &str,
    c: char,
    count_repeat: usize,
    hash_map: &mut HashMap<u32, String>,
) -> Option<u32> {
    let value = format!("{c}").repeat(count_repeat);
    let mut result = None;
    (1..=1000)
        .map(|i| n + i)
        .map(|idx| {
            let digest = if let Some(digest) = hash_map.get(&idx) {
                digest.clone()
            } else {
                let digest = md5::compute(format!("{input}{}", idx));
                let digest = format!("{:x}", digest);
                hash_map.insert(idx, digest.clone());
                digest
            };
            (idx, digest)
        })
        .any(|(idx, digest)| {
            if digest.contains(&value) {
                result = Some(idx);
                true
            } else {
                false
            }
        });
    result
}

fn is_triple(n: u32, input: &str) -> Option<char> {
    let digest = md5::compute(format!("{input}{n}"));
    let digest = format!("{:x}", digest);
    let mut result = None;
    digest.chars().collect::<Vec<_>>().windows(3).any(|c| {
        if c[0] == c[1] && c[1] == c[2] {
            result = Some(c[0]);
            true
        } else {
            false
        }
    });
    result
}

fn find_hashes(count: usize, input: &str) -> Vec<u32> {
    let mut result = vec![];
    let mut idx = 0;
    let mut hash_map = HashMap::new();
    while result.len() < count {
        if let Some(c) = is_triple(idx, input) {
            if find_match(idx, input, c, 5, &mut hash_map).is_some() {
                result.push(idx);
            }
        }
        idx += 1;
    }
    result
}

fn main() {
    let hashes = find_hashes(64, "ngcjuoqr");
    println!("Part 1 = {}", hashes.last().unwrap());
}

#[cfg(test)]
mod day14_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { 1, 17, 18, 19, 25, 39, 92 },
        expected = { None, None, Some('8'), None, None, Some('e'), Some('9') }
    )]
    fn is_triple(input: u32, expected: Option<char>) {
        assert_eq!(is_triple(input, "abc"), expected);
    }

    #[parameterized(
        input = { 18, 39, 92 },
        c = { '8', 'e', '9' },
        expected = { None, Some(816), Some(200) }
    )]
    fn find_match(input: u32, c: char, expected: Option<u32>) {
        assert_eq!(
            find_match(input, "abc", c, 5, &mut HashMap::new()),
            expected
        );
    }

    #[test]
    fn part1() {
        let hashes = find_hashes(64, "abc");
        assert_eq!(*hashes.last().unwrap(), 22728);
    }
}
