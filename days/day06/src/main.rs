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

fn least_frequent_char(input: &str) -> char {
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
    let max_freq = frequencies.keys().min().unwrap();
    **frequencies.get(max_freq).unwrap()
}

fn decode<T>(input: Vec<String>, freq_fun: T) -> String
where
    T: Fn(&str) -> char,
{
    input.iter().map(|s| freq_fun(s)).collect()
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
    println!("Part 1 = {}", decode(input.clone(), most_frequent_char));
    println!("Part 2 = {}", decode(input, least_frequent_char));
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
        assert_eq!(decode(input, most_frequent_char), "easter".to_string());
    }

    #[test]
    fn part2() {
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
        assert_eq!(decode(input, least_frequent_char), "advent".to_string());
    }
}
