use common::read_input;

#[derive(Debug)]
enum OutputOrBot {
    Output(usize),
    Bot(usize),
}

#[allow(dead_code)]
struct Output {
    id: usize,
    value: u32,
}

#[derive(Debug)]
struct Bot {
    id: usize,
    value1: Option<u32>,
    value2: Option<u32>,
    low: OutputOrBot,
    high: OutputOrBot,
}

impl From<&str> for Bot {
    fn from(value: &str) -> Self {
        let mut input = value.split_whitespace();
        input.next();
        let id = input.next().unwrap().parse::<usize>().unwrap();
        input.next();
        input.next();
        input.next();
        let low = match input.next().unwrap() {
            "output" => OutputOrBot::Output(input.next().unwrap().parse::<usize>().unwrap()),
            "bot" => OutputOrBot::Bot(input.next().unwrap().parse::<usize>().unwrap()),
            s => panic!("Unknown target for low {s}"),
        };
        input.next();
        input.next();
        input.next();
        let high = match input.next().unwrap() {
            "output" => OutputOrBot::Output(input.next().unwrap().parse::<usize>().unwrap()),
            "bot" => OutputOrBot::Bot(input.next().unwrap().parse::<usize>().unwrap()),
            s => panic!("Unknown target for high {s}"),
        };
        Self {
            id,
            value1: None,
            value2: None,
            low,
            high,
        }
    }
}

impl Bot {
    fn process(&mut self, value: u32, bots: &mut Vec<Bot>, outputs: &mut Vec<Output>) {
        if self.value1.is_none() {
            self.value1 = Some(value);
        } else if self.value2.is_none() {
            self.value2 = Some(value);
        } else {
            panic!(
                "Bot {} already handles 2 values {}, {}",
                self.id,
                self.value1.unwrap(),
                self.value2.unwrap()
            );
        };
        if let Some(value1) = self.value1 {
            if let Some(value2) = self.value2 {
                let value = value1.min(value2);
                match self.low {
                    OutputOrBot::Output(id) => outputs.push(Output { id, value }),
                    OutputOrBot::Bot(id) => {
                        let idx = bots.iter().position(|b| b.id == id).unwrap();
                        let mut low_bot = bots.remove(idx);
                        low_bot.process(value, bots, outputs);
                        bots.insert(idx, low_bot);
                    }
                }
                let value = value1.max(value2);
                match self.high {
                    OutputOrBot::Output(id) => outputs.push(Output { id, value }),
                    OutputOrBot::Bot(id) => {
                        let idx = bots.iter().position(|b| b.id == id).unwrap();
                        let mut high_bot = bots.remove(idx);
                        high_bot.process(value, bots, outputs);
                        bots.insert(idx, high_bot);
                    }
                }
            }
        }
    }
}

struct ValueInput {
    val: u32,
    bot: usize,
}

impl From<&str> for ValueInput {
    fn from(value: &str) -> Self {
        let mut input = value.split_whitespace();
        input.next();
        let val = input.next().unwrap().parse::<u32>().unwrap();
        input.next();
        input.next();
        input.next();
        let bot = input.next().unwrap().parse::<usize>().unwrap();
        Self { val, bot }
    }
}

struct Factory {
    bots: Vec<Bot>,
    value_inputs: Vec<ValueInput>,
    outputs: Vec<Output>,
}

impl From<&str> for Factory {
    fn from(value: &str) -> Self {
        let (bots, value_inputs) =
            value
                .lines()
                .fold((vec![], vec![]), |(mut bots, mut value_inputs), l| {
                    if l.starts_with("value") {
                        value_inputs.push(ValueInput::from(l));
                    } else if l.starts_with("bot") {
                        bots.push(Bot::from(l));
                    };
                    (bots, value_inputs)
                });
        Self {
            bots,
            value_inputs,
            outputs: vec![],
        }
    }
}

impl Factory {
    fn process(&mut self) {
        self.value_inputs.iter().for_each(|v| {
            let idx = self.bots.iter().position(|b| b.id == v.bot).unwrap();
            let mut bot = self.bots.remove(idx);
            bot.process(v.val, &mut self.bots, &mut self.outputs);
            self.bots.insert(idx, bot);
        });
    }

    fn find_processor(&self, value1: u32, value2: u32) -> usize {
        self.bots
            .iter()
            .find(|b| {
                (b.value1 == Some(value1) && b.value2 == Some(value2))
                    || (b.value2 == Some(value1) && b.value1 == Some(value2))
            })
            .unwrap()
            .id
    }
}

fn main() {
    let input = read_input("day10.txt");
    let mut factory = Factory::from(input.as_str());
    factory.process();
    println!("Part 1 = {}", factory.find_processor(61, 17));
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2"#;
        let mut factory = Factory::from(input);
        factory.process();
        assert_eq!(factory.find_processor(5, 2), 2);
    }
}
