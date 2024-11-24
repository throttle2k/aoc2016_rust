fn find_hashes<T>(input: &str, n_zeroes: usize, n_hashes: usize, is_valid: T) -> Vec<String>
where
    T: Fn(&str, &[String]) -> bool,
{
    let mut current = 0;
    let mut found_hashes = vec![];
    while found_hashes.len() < n_hashes {
        let digest = md5::compute(format!("{input}{current}"));
        let hex = format!("{:x}", digest);
        if hex.starts_with(&"0".repeat(n_zeroes)) && is_valid(&hex, &found_hashes) {
            found_hashes.push(hex);
        };
        current += 1;
    }
    found_hashes
}

fn find_password(input: &str) -> String {
    let mut hashes = find_hashes(input, 5, 8, |_, _| true);
    hashes
        .iter_mut()
        .map(|hash| hash.remove(5))
        .collect::<String>()
}

fn find_second_password(input: &str) -> String {
    let hashes = find_hashes(input, 5, 8, |s, h| {
        let c = s.as_bytes()[5] as char;
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => {
                h.iter().all(|hash| hash.chars().nth(5).unwrap() != c)
            }
            _ => false,
        }
    });
    let password = hashes.iter().fold(['_'; 8], |mut password, hash| {
        let position = hash.chars().nth(5).unwrap();
        let char = hash.chars().nth(6).unwrap();
        let position = format!("{position}").parse::<usize>().unwrap();
        password[position] = char;
        password
    });
    password.iter().collect::<String>()
}

fn main() {
    let input = "ffykfhsq";
    println!("Part 1 = {}", find_password(input));
    println!("Part 2 = {}", find_second_password(input));
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "abc";
        assert_eq!(find_password(input), "18f47a30".to_string())
    }

    #[test]
    fn part2() {
        let input = "abc";
        assert_eq!(find_second_password(input), "05ace8e3".to_string())
    }
}
