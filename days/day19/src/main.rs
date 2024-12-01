#[derive(Clone, Debug)]
struct Elf(usize, u32);

#[derive(Clone, Debug)]
struct Circle {
    elves: Vec<Elf>,
}

impl Circle {
    fn new(num_elves: usize) -> Self {
        let elves = (1..=num_elves).map(|i| Elf(i, 0)).collect();
        Self { elves }
    }

    fn turn(&self) -> Circle {
        let mut elves = vec![];
        let mut chunks = self.elves.chunks_exact(2);
        while let Some([first, _second]) = chunks.next() {
            elves.push(Elf(first.0, first.1 * 2));
        }
        if chunks.remainder().len() > 0 {
            elves.remove(0);
            elves.push(Elf(chunks.remainder()[0].0, chunks.remainder()[0].1 * 2));
        }
        Circle { elves }
    }

    fn play(&self) -> Elf {
        let mut circle = self.clone();
        let mut current = 0;
        let mut next;
        let mut count = 0;
        while circle.elves.len() > 1 && count < 3001330 {
            next = (current + 1) % circle.elves.len();
            circle = circle.turn();
            current = next;
            count += 1;
        }
        circle.elves.get(0).unwrap().clone()
    }
}

fn main() {
    let circle = Circle::new(3001330);
    println!("Part 1 = {}", circle.play().0);
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn part1() {
        let circle = Circle::new(5);
        assert_eq!(circle.play().0, 3);
    }
}
