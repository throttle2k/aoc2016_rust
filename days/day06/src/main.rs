use std::collections::HashMap;

use common::read_input;

fn most_frequent_char(input: &str) -> char {
    let frequencies = input
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut frequencies, c| {
            frequencies.entry(c).and_modify(|f| *f += 1).or_insert(1);
            frequencies
        });
    let frequencies = frequencies
        .iter()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<&usize, &char>>();
    let max_freq = frequencies.keys().max().unwrap();
    **frequencies.get(max_freq).unwrap()
}

fn decode(input: Vec<String>) -> String {
    input.iter().map(|s| most_frequent_char(s)).collect()
}

fn transpose_strings(input: &[&str]) -> Vec<String> {
    let n_cols = input[0].len();
    let mut rows = vec![String::new(); n_cols];
    input
        .iter()
        .for_each(|s| s.chars().enumerate().for_each(|(i, c)| rows[i].push(c)));
    rows
}

fn main() {
    let input = read_input("day06.txt");
    let input = input.trim().lines().map(|l| l.trim()).collect::<Vec<_>>();
    let input = transpose_strings(&input);
    println!("Part 1 = {}", decode(input));
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;
        let input = input.trim().lines().map(|l| l.trim()).collect::<Vec<_>>();
        let input = transpose_strings(&input);
        assert_eq!(decode(input), "easter".to_string());
    }
}
