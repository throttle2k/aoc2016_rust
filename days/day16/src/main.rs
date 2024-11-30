use core::panic;

fn calculate_checksum(input: &str) -> String {
    let checksum = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| match (c[0], c[1]) {
            ('1', '1') | ('0', '0') => '1',
            ('0', '1') | ('1', '0') => '0',
            (c1, c2) => panic!("Unknown characters: {c1} {c2}"),
        })
        .collect::<String>();
    if checksum.len() % 2 == 1 {
        checksum
    } else {
        calculate_checksum(&checksum)
    }
}

fn generate_data(input: &str) -> String {
    let reverse = input
        .chars()
        .rev()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            c => panic!("Wrong character: {c}"),
        })
        .map(|i| format!("{i}"))
        .collect::<String>();
    format!("{input}0{reverse}")
}

fn generate_data_for_size(input: &str, size: usize) -> String {
    let mut data = generate_data(input);
    while data.len() < size {
        data = generate_data(&data);
    }
    data.chars().take(size).collect()
}

fn main() {
    let data = generate_data_for_size("00111101111101000", 272);
    println!("Part 1 = {}", calculate_checksum(&data));
    let data = generate_data_for_size("00111101111101000", 35651584);
    println!("Part 2 = {}", calculate_checksum(&data));
}

#[cfg(test)]
mod day16_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "1", "0", "11111", "111100001010" },
        expected = { "100".to_owned(), "001".to_owned(), "11111000000".to_owned(), "1111000010100101011110000".to_owned() }
    )]
    fn test_generate_data(input: &str, expected: String) {
        assert_eq!(generate_data(input), expected);
    }

    #[test]
    fn test_generate_data() {
        assert_eq!(
            generate_data_for_size("10000", 20),
            "10000011110010000111".to_owned()
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(calculate_checksum("110010110100"), "100".to_owned());
    }

    #[test]
    fn part1() {
        let data = generate_data_for_size("10000", 20);
        assert_eq!(calculate_checksum(&data), "01100".to_owned());
    }
}
