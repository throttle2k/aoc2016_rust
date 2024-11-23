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

fn transpose_input(input: &str) -> String {
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let num_columns = rows[0].len();

    let mut transposed = vec![vec![]; num_columns];
    for row in rows {
        for (i, &value) in row.iter().enumerate() {
            transposed[i].push(value);
        }
    }

    transposed
        .into_iter()
        .flat_map(|column| column)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.join(" "))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let input = read_input("day03.txt");
    let count_triangles: usize = input
        .lines()
        .filter_map(|l| if is_triangle(l) { Some(1) } else { None })
        .sum();
    println!("Part1 = {}", count_triangles);
    let count_triangles: usize = transpose_input(&input)
        .lines()
        .filter_map(|l| if is_triangle(l) { Some(1) } else { None })
        .sum();
    println!("Part2 = {}", count_triangles);
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "5 10 25";
        assert!(!is_triangle(input));
    }

    #[test]
    fn input_can_be_read_as_rows() {
        let input = r#"101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603"#;
        assert_eq!(
            transpose_input(input),
            r#"101 102 103
201 202 203
301 302 303
401 402 403
501 502 503
601 602 603"#
        );
    }
}
