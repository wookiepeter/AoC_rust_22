#![allow(dead_code)]
use std::{collections::HashSet, fs};

pub fn run() {
    let input = fs::read_to_string("./assets/input_09.txt").expect("File not found!");
    let _test_input = fs::read_to_string("./assets/input_09_test.txt").expect("File not found!");
    let _part2_test_input =
        fs::read_to_string("./assets/input_09_test_2.txt").expect("File not found!");

    // part1(&_test_input);

    // part1(&input);
    // part2(&_part2_test_input);
    part2(&input);
}

fn part1(input: &str) {
    let mut previous_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    previous_positions.insert(tail);
    for line in input.lines() {
        let dir = match line.get(0..1).unwrap() {
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => (0, 0),
        };

        let num = line.get(2..).unwrap().parse::<i32>().unwrap();

        for _ in 0..num {
            let old_pos = head.clone();
            head.add(&dir);
            if head.distance(&tail) > 1 {
                tail = old_pos;
                previous_positions.insert(tail);
            }
        }

        println!(
            "moving {} for {} steps. new positions - head: {:?} tail {:?}",
            line.get(0..1).unwrap(),
            num,
            head,
            tail
        );
    }
    println!(
        "The rope visited a total of {} positions.",
        previous_positions.len()
    );
}

fn part2(input: &str) {
    let mut previous_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail_positions = vec![(0, 0); 9];
    previous_positions.insert((0, 0));

    for line in input.lines() {
        let dir = match line.get(0..1).unwrap() {
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => (0, 0),
        };

        let num = line.get(2..).unwrap().parse::<i32>().unwrap();
        // println!("moving {} for {} steps.", line.get(0..1).unwrap(), num,);

        for _ in 0..num {
            // move head
            head.add(&dir);

            // move first tail position (depends on head)
            if head.distance(&tail_positions[0]) > 1 {
                tail_positions[0] = tail_positions[0].move_towards(&head);
            }
            // move other tail positions (depends on previous knot)
            for i in 1..tail_positions.len() {
                if tail_positions[i - 1].distance(&tail_positions[i]) > 1 {
                    tail_positions[i] = tail_positions[i].move_towards(&tail_positions[i - 1]);
                }
            }
            previous_positions.insert(tail_positions[8]);
        }

        // println!("head: {:?} - tail positions: {:?}", head, tail_positions)
    }
    println!(
        "The ropes tail visited a total of {} positions.",
        previous_positions.len()
    );
}

pub trait RopeDistance<T> {
    fn distance(&self, other: &T) -> i32;

    fn add(&mut self, other: &T);

    fn move_towards(&self, target: &T) -> T;
}

impl RopeDistance<(i32, i32)> for (i32, i32) {
    fn distance(&self, other: &(i32, i32)) -> i32 {
        i32::max((self.0 - other.0).abs(), (self.1 - other.1).abs())
    }

    fn add(&mut self, other: &(i32, i32)) {
        self.0 += other.0;
        self.1 += other.1;
    }

    fn move_towards(&self, target: &(i32, i32)) -> (i32, i32) {
        (
            self.0 + (target.0 - self.0).signum(),
            self.1 + (target.1 - self.1).signum(),
        )
    }
}
