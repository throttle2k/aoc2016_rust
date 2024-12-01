use common::read_input;

fn check_trap(prev: &str) -> bool {
    let left = prev.chars().nth(0).unwrap();
    let center = prev.chars().nth(1).unwrap();
    let right = prev.chars().nth(2).unwrap();
    let first_rule = left == '^' && center == '^' && right == '.';
    let second_rule = left == '.' && center == '^' && right == '^';
    let third_rule = left == '^' && center == '.' && right == '.';
    let fourth_rule = left == '.' && center == '.' && right == '^';
    first_rule || second_rule || third_rule || fourth_rule
}

fn next_row(from: &str) -> String {
    let row_len = from.len();
    (0..row_len)
        .map(|i| match i {
            0 => from.chars().nth(1).unwrap() == '^',
            n if n == row_len - 1 => from.chars().nth(row_len - 2).unwrap() == '^',
            n => check_trap(&from[n - 1..n + 2]),
        })
        .map(|tile| match tile {
            true => '^',
            false => '.',
        })
        .collect()
}

fn produce_map(from: &str, rows: usize) -> String {
    let map = (0..rows).fold(vec![String::from(from.trim())], |mut v, _| {
        let next = next_row(v.last().unwrap());
        v.push(next);
        v
    });
    map.join("\n")
}

fn count_safe(map: &str) -> usize {
    map.lines()
        .map(|l| l.chars().filter(|c| *c == '.').count())
        .sum()
}

fn main() {
    let input = read_input("day18.txt");
    let map = produce_map(input.as_str(), 39);
    println!("Part 1 = {}", count_safe(&map));
    let map = produce_map(input.as_str(), 399999);
    println!("Part 2 = {}", count_safe(&map));
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn test_next_row() {
        let next = next_row("..^^.");
        assert_eq!(next, ".^^^^".to_owned());
        let next = next_row(&next);
        assert_eq!(next, "^^..^".to_owned());
    }

    #[test]
    fn test_produce_map() {
        let expected = r#".^^.^.^^^^
^^^...^..^
^.^^.^.^^.
..^^...^^^
.^^^^.^^.^
^^..^.^^..
^^^^..^^^.
^..^^^^.^^
.^^^..^.^^
^^.^^^..^^"#;
        let input = ".^^.^.^^^^";
        assert_eq!(produce_map(input, 9), expected.to_owned());
    }

    #[test]
    fn part1() {
        let input = ".^^.^.^^^^";
        let map = produce_map(input, 9);
        assert_eq!(count_safe(&map), 38);
    }
}
