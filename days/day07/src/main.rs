use common::read_input;
use regex::Regex;

#[derive(Debug)]
struct Address(String);

impl Address {
    fn contains_abba(&self) -> bool {
        self.0.chars().collect::<Vec<_>>().windows(4).any(|chars| {
            chars[0] != chars[1] && chars[0] == chars[3] && chars[1] == chars[2]
        })
    }

    fn find_abas(&self) -> Vec<String> {
        self.0.chars().collect::<Vec<_>>().windows(3).filter(|chars| chars[0] == chars[2] && chars[0] != chars[1]).map(|chars| format!("{}{}{}", chars[0], chars[1], chars[2])).collect::<Vec<_>>()
    }

    fn contains(&self, bab: &str) -> bool {
        self.0.contains(bab)
    }
}


#[derive(Debug)]
struct IPv7 {
    supernet_sequences: Vec<Address>,
    hypernet_sequences: Vec<Address>,
}

impl From<&str> for IPv7 {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"\[\w+\]").unwrap();
        let hypernet_sequences = re
            .find_iter(value)
            .map(|m| Address(m.as_str().to_owned()))
            .collect::<Vec<_>>();
        let normal_addresses = re.replace_all(value, " ").split(" ").map(|s| Address(s.to_string())).collect::<Vec<_>>();
        Self {
            supernet_sequences: normal_addresses, hypernet_sequences
        }
    }
}

impl IPv7 {
    fn supports_tls(&self) -> bool {
        self.supernet_sequences.iter().any(|a| a.contains_abba()) && self.hypernet_sequences.iter().all(|a| !a.contains_abba())
    }

    fn supports_ssl(&self) -> bool {
        let abas = self.supernet_sequences.iter().flat_map(|a| a.find_abas()).collect::<Vec<_>>();
        abas.iter().any(|aba| {
            let bab = format!("{}{}{}", aba.chars().nth(1).unwrap(), aba.chars().nth(0).unwrap(), aba.chars().nth(1).unwrap());
            self.hypernet_sequences.iter().any(|a| a.contains(&bab))
        })
    }
}

fn main() {
    let input = read_input("day07.txt");
    let ips_with_tls = input.trim().lines().map(|l| IPv7::from(l.trim())).filter(|ip| ip.supports_tls()).count();
    println!("Part1 = {}", ips_with_tls);
    let ips_with_ssl = input.trim().lines().map(|l| IPv7::from(l.trim())).filter(|ip| ip.supports_ssl()).count();
    println!("Part2 = {}", ips_with_ssl);
}

#[cfg(test)]
mod day07_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "abba[mnop]qrst", "abcd[bddb]xyyx", "aaaa[qwer]tyui", "ioxxoj[asdfgh]zxcvbn", "pippo[abc]cc[poipoi]" },
        expected = { true, false, false, true, false } 
    )]
    fn part1(input: &str, expected: bool) {
        let address = IPv7::from(input);
        assert_eq!(address.supports_tls(), expected);
    }

    #[parameterized(
        input = { "aba[bab]xyz", "xyx[xyx]xyx", "aaa[kek]eke", "zazbz[bzb]cdb" },
        expected = { true, false, true, true } 
    )]
    fn part2(input: &str, expected: bool) {
        let address = IPv7::from(input);
        assert_eq!(address.supports_ssl(), expected);
    }
}
