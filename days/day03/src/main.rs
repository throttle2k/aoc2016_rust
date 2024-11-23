use common::read_input;

fn check_triangle(num: u32, rest: Vec<u32>) -> bool {
    num < rest.iter().sum()
}

fn is_triangle(input: &str) -> bool {
    let input = input.trim().split_whitespace().collect::<Vec<_>>();
    (0..input.len()).all(|i| {
        let mut input = input
            .clone()
            .iter()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let num = input.remove(i);
        check_triangle(num, input)
    })
}

fn main() {
    let input = read_input("day03.txt");
    let count_triangles: usize = input
        .lines()
        .filter_map(|l| if is_triangle(l) { Some(1) } else { None })
        .sum();
    println!("Part1 = {}", count_triangles);
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "5 10 25";
        assert!(!is_triangle(input));
    }
}
