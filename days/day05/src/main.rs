fn find_hashes(input: &str, n_zeroes: usize, n_hashes: usize) -> Vec<String> {
    let mut current = 0;
    let mut found_hashes = vec![];
    while found_hashes.len() < n_hashes {
        let digest = md5::compute(format!("{input}{current}"));
        let hex = format!("{:x}", digest);
        if hex.starts_with(&"0".repeat(n_zeroes)) {
            found_hashes.push(hex);
        };
        current += 1;
    }
    found_hashes
}

fn find_password(input: &str, code_len: usize, n_char: usize) -> String {
    let mut hashes = find_hashes(input, n_char - 1, code_len);
    hashes
        .iter_mut()
        .map(|hash| hash.remove(n_char - 1))
        .collect::<String>()
}

fn main() {
    let input = "ffykfhsq";
    println!("Part 1 = {}", find_password(input, 8, 6));
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "abc";
        assert_eq!(find_password(input, 8, 6), "18f47a30".to_string())
    }
}
