use common::read_input;

struct Marker {
    n_chars: usize,
    repeat: usize,
}

impl From<&str> for Marker {
    fn from(value: &str) -> Self {
        let value = value.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let (n_chars, repeat) = value.split_once('x').unwrap();
        let (n_chars, repeat) = (n_chars.parse().unwrap(), repeat.parse().unwrap());
        Self { n_chars, repeat }
    }
}

fn decompress(input: &str, nested: bool) -> String {
    let mut result = String::new();
    let mut marker: Option<Marker> = None;
    let mut marker_str = String::new();
    let mut in_marker = false;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            marker_str.push(c);
            in_marker = true;
        } else if in_marker && c == ')' {
            marker_str.push(c);
            marker = Some(Marker::from(marker_str.as_str()));
            marker_str = String::new();
            in_marker = false;
        } else if in_marker {
            marker_str.push(c);
        } else if !in_marker && !c.is_whitespace() {
            result.push(c);
        }
        if let Some(m) = &marker {
            let mut repeatee = String::new();
            (0..m.n_chars).for_each(|_| repeatee.push(chars.next().unwrap()));
            repeatee = if nested {
                decompress(repeatee.as_str(), nested)
            } else {
                repeatee
            };
            result.push_str(&repeatee.repeat(m.repeat));
            marker = None;
        }
    }
    result
}

fn main() {
    let input = read_input("day09.txt");
    println!("Part 1 = {}", decompress(input.as_str(), false).len());
    println!("Part 2 = {}", decompress(input.as_str(), true).len());
}

#[cfg(test)]
mod day09_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "ADVENT", "A(1x5)BC" , "(3x3)XYZ", "A(2x2)BCD(2x2)EFG", "(6x1)(1x3)A", "X(8x2)(3x3)ABCY" },
        expected = { "ADVENT", "ABBBBBC", "XYZXYZXYZ", "ABCBCDEFEFG", "(1x3)A", "X(3x3)ABC(3x3)ABCY" },
        length = { 6, 7, 9, 11, 6, 18 }
    )]
    fn part1(input: &str, expected: &str, length: usize) {
        let decompressed = decompress(input, false);
        assert_eq!(decompressed, expected);
        assert_eq!(decompressed.len(), length);
    }

    #[parameterized(
        input = { "(3x3)XYZ", "X(8x2)(3x3)ABCY" , "(27x12)(20x12)(13x14)(7x10)(1x12)A", "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN" },
        length = { 9, 20, 241920, 445 }
    )]
    fn part2(input: &str, length: usize) {
        let decompressed = decompress(input, true);
        assert_eq!(decompressed.len(), length);
    }
}
