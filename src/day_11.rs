#![allow(dead_code)]
use std::fs;

use itertools::Itertools;

// requires input files to have LF line endings!
pub fn run() {
    let _test_input = fs::read_to_string("./assets/input_11_test.txt").expect("File not found!");
    let input = fs::read_to_string("./assets/input_11.txt").expect("File not found!");

    // part1(&_test_input);
    part2(&input);
}

fn part1(input: &str) {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|section| Monkey::new(section))
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let inspected_items: Vec<(usize, u64)> = monkeys[i]
                .items
                .iter()
                .map(|item| monkeys[i].process_item(*item))
                .collect();
            monkeys[i].inspect_count += monkeys[i].items.len() as u64;
            inspected_items
                .iter()
                .for_each(|item| monkeys[item.0].items.push(item.1));
            monkeys[i].items.clear();
        }
    }
    let monkey_business: u64 = monkeys
        .iter()
        .map(|monkey| monkey.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product();

    println!("Monkey business is {}", monkey_business);
}

fn part2(input: &str) {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|section| Monkey::new(section))
        .collect();

    let modulo_magic: u64 = monkeys.iter().map(|monkey| monkey.test_value).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let inspected_items: Vec<(usize, u64)> = monkeys[i]
                .items
                .iter()
                .map(|item| monkeys[i].process_item_for_part2(*item, modulo_magic))
                .collect();
            monkeys[i].inspect_count += monkeys[i].items.len() as u64;
            inspected_items
                .iter()
                .for_each(|item| monkeys[item.0].items.push(item.1));
            monkeys[i].items.clear();
        }
    }
    let monkey_business: u64 = monkeys
        .iter()
        .map(|monkey| monkey.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product();

    println!("Monkey business is {}", monkey_business);
}

struct Monkey {
    items: Vec<u64>,
    operation: fn(u64, u64) -> u64,
    op_value: Option<u64>,
    test_value: u64,
    target_if_true: usize,
    target_if_false: usize,
    inspect_count: u64,
}

fn mul(v1: u64, v2: u64) -> u64 {
    v1 * v2
}

fn add(v1: u64, v2: u64) -> u64 {
    v1 + v2
}

impl Monkey {
    fn new(description: &str) -> Monkey {
        let mut lines = description.lines();
        lines.next();

        let items: Vec<u64> = cleanup_line(lines.next().unwrap(), ": ")
            .split(", ")
            .map(|value| value.parse::<u64>().unwrap())
            .collect();

        let op_slices: Vec<&str> = cleanup_line(lines.next().unwrap(), "= ")
            .split_whitespace()
            .collect();

        let operation = match op_slices[1] {
            "*" => mul,
            "+" => add,
            _ => panic!("Should always find an operation"),
        };

        let op_value = match (op_slices[0], op_slices[2]) {
            ("old", "old") => None,
            ("old", value) | (value, "old") => Some(value.parse::<u64>().unwrap()),
            _ => panic!("Should always find one of the above cases!"),
        };

        let test_value = cleanup_line(lines.next().unwrap(), "by ")
            .parse::<u64>()
            .unwrap();

        let target_if_true = cleanup_line(lines.next().unwrap(), "monkey ")
            .parse::<usize>()
            .unwrap();

        let target_if_false = cleanup_line(lines.next().unwrap(), "monkey ")
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            operation,
            op_value,
            test_value,
            target_if_true,
            target_if_false,
            inspect_count: 0,
        }
    }

    fn process_item(&self, item: u64) -> (usize, u64) {
        let mut new_worry_level = match self.op_value {
            Some(value) => (self.operation)(item, value),
            None => (self.operation)(item, item),
        };
        new_worry_level /= 3;
        match new_worry_level % self.test_value == 0 {
            true => (self.target_if_true, new_worry_level),
            false => (self.target_if_false, new_worry_level),
        }
    }

    // Due to the large number of repetitions we are supposed to simulate now,
    // we have to find a way to keep the worry levels to a reasonabley low number
    // while still accurately simulating the monkey behaviour. We can do this by taking all the
    // test_values (which need to produce accurate modulo values from the worry_levels) and
    // combine them into one common number by multiplying them (modulo_magic).
    // Now we can safely use `worry_level = worry_level % modulo_magic` to keep the worry_level
    // in managable ranges while still receiving the correct results during the actual test
    // operation, since worry_level % modulo_magic == worry_level % test_value.

    fn process_item_for_part2(&self, item: u64, modulo_magic: u64) -> (usize, u64) {
        let mut new_worry_level = match self.op_value {
            Some(value) => (self.operation)(item, value),
            None => (self.operation)(item, item),
        };
        new_worry_level = new_worry_level % modulo_magic;
        match new_worry_level % self.test_value == 0 {
            true => (self.target_if_true, new_worry_level),
            false => (self.target_if_false, new_worry_level),
        }
    }
}

fn cleanup_line<'a>(line: &'a str, marker: &str) -> &'a str {
    let (_, content) = line.split_once(marker).unwrap();
    content
}
