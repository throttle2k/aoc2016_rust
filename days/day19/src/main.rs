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

    fn solve_part_2(num_elves: usize) -> usize {
        let mut mst = num_elves;
        let mut power_of_3 = 1;
        while mst > 2 {
            mst /= 3;
            power_of_3 *= 3;
        }

        if power_of_3 == num_elves {
            num_elves
        } else if mst == 1 {
            num_elves - power_of_3
        } else {
            num_elves * 2 - power_of_3 * 3
        }
    }
}

fn main() {
    let circle = Circle::new(3001330);
    println!("Part 1 = {}", circle.play().0);
    println!("Part 2 = {}", Circle::solve_part_2(3001330));
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn part1() {
        let circle = Circle::new(5);
        assert_eq!(circle.play().0, 3);
    }

    #[test]
    fn part2() {
        assert_eq!(Circle::solve_part_2(5), 2);
    }
}
