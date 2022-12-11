use anyhow::Result;

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    div: u32,
    true_target: usize,
    false_target: usize,
    operand: char,
    operators: (String, String)
}

impl Monkey {
    fn decide_target(self: &Self, item: u32) -> (usize, u32) {
        let parsed_operators = match (self.operators.0.as_str(), self.operators.1.as_str()) {
            ("old", "old") => (item, item),
            ("old", num) => (item, num.parse::<u32>().unwrap()),
            _ => (0, 0)
        };
        let new_worry_level = match self.operand {
            '+' => parsed_operators.0 + parsed_operators.1,
            '*' => parsed_operators.0 * parsed_operators.1,
            _ => 0
        } / 3;
        match new_worry_level % self.div {
            0 => (self.true_target, new_worry_level),
            _ => (self.false_target, new_worry_level)
        }
    }

    fn decide_target_2(self: &Self, item: u32, worry_mod: u32) -> (usize, u32) {
        let parsed_operators = match (self.operators.0.as_str(), self.operators.1.as_str()) {
            ("old", "old") => (item % worry_mod, item % worry_mod),
            ("old", num) => (item % worry_mod, num.parse::<u32>().unwrap()),
            _ => (0, 0)
        };
        let new_worry_level: u64 = match self.operand {
            '+' => parsed_operators.0 as u64 + parsed_operators.1 as u64,
            '*' => parsed_operators.0 as u64 * parsed_operators.1 as u64,
            _ => 0
        } % worry_mod as u64;
        match new_worry_level % self.div as u64 {
            0 => (self.true_target, new_worry_level as u32),
            _ => (self.false_target, new_worry_level as u32)
        }
    }

    fn catch(self: &mut Self, item: u32) -> () {
        self.items.push(item);
    }
}

fn part01() -> Result<u32> {
    let mut monkeys = std::fs::read_to_string("./data/day11.input")?
        .trim()
        .split("\n\n")
        .into_iter()
        .map(|block| {
            let lines = block.split("\n").collect::<Vec<&str>>();
            let op: Vec<&str> = lines[2].split("= ").into_iter().nth(1).unwrap().split_whitespace().collect();
            Monkey {
                items: lines[1].split_terminator(&[':', ',']).filter_map(|num| num.trim().parse::<u32>().ok()).into_iter().collect(),
                div: lines[3].split_whitespace().last().unwrap().parse().unwrap(),
                true_target: lines[4].split_whitespace().last().unwrap().parse().unwrap(),
                false_target: lines[5].split_whitespace().last().unwrap().parse().unwrap(),
                operand: op[1].chars().next().unwrap(),
                operators: (op[0].to_string(), op[2].to_string()) }
        })
    .collect::<Vec<Monkey>>();

    let mut inspected_items: Vec<u32> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            for item_idx in 0..monkeys[monkey].items.len() {
                let item = monkeys[monkey].items[item_idx];
                let (target_monkey, new_worry_level) = monkeys[monkey].decide_target(item);
                monkeys[target_monkey].catch(new_worry_level);
            }
            inspected_items[monkey] += monkeys[monkey].items.len() as u32;
            monkeys[monkey].items.clear();
        }
    }

    inspected_items.sort_by(|a, b| b.cmp(a));
    Ok(inspected_items[0] * inspected_items[1])
}

fn part02() -> Result<u64> {
    let mut monkeys = std::fs::read_to_string("./data/day11.input")?
        .trim()
        .split("\n\n")
        .into_iter()
        .map(|block| {
            let lines = block.split("\n").collect::<Vec<&str>>();
            let op: Vec<&str> = lines[2].split("= ").into_iter().nth(1).unwrap().split_whitespace().collect();
            Monkey {
                items: lines[1].split_terminator(&[':', ',']).filter_map(|num| num.trim().parse::<u32>().ok()).into_iter().collect(),
                div: lines[3].split_whitespace().last().unwrap().parse().unwrap(),
                true_target: lines[4].split_whitespace().last().unwrap().parse().unwrap(),
                false_target: lines[5].split_whitespace().last().unwrap().parse().unwrap(),
                operand: op[1].chars().next().unwrap(),
                operators: (op[0].to_string(), op[2].to_string()) }
        })
    .collect::<Vec<Monkey>>();

    let worry_mod = monkeys.iter().fold(1, |acc, monkey| acc * monkey.div);
    let mut inspected_items: Vec<u32> = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for monkey in 0..monkeys.len() {
            for item_idx in 0..monkeys[monkey].items.len() {
                let item = monkeys[monkey].items[item_idx];
                let (target_monkey, new_worry_level) = monkeys[monkey].decide_target_2(item, worry_mod);
                monkeys[target_monkey].catch(new_worry_level);
            }
            inspected_items[monkey] += monkeys[monkey].items.len() as u32;
            monkeys[monkey].items.clear();
        }
    }
    println!("{:?}", inspected_items);

    inspected_items.sort_by(|a, b| b.cmp(a));
    Ok(inspected_items[0] as u64 * inspected_items[1] as u64)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
