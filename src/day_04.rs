#![allow(dead_code)]
use std::fs;
use text_io;

pub fn run() {
    let input = fs::read_to_string("./assets/input_04.txt").expect("Could not load file!");

    println!(
        "The total numbers of  pairs containing another is: {}",
        part1(&input)
    );

    println!(
        "The total numbers of pairs overlapping another is: {}",
        part2(&input)
    );
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (v1, v2, v3, v4): (i32, i32, i32, i32);
            text_io::scan!(line.bytes() => "{}-{},{}-{}", v1, v2, v3, v4);
            let lhs = (v1, v2);
            let rhs = (v3, v4);
            if is_inside(lhs, rhs) || is_inside(rhs, lhs) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

fn is_inside(inside: (i32, i32), outside: (i32, i32)) -> bool {
    inside.0 >= outside.0 && inside.1 <= outside.1
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (v1, v2, v3, v4): (i32, i32, i32, i32);
            text_io::scan!(line.bytes() => "{}-{},{}-{}", v1, v2, v3, v4);
            let lhs = (v1, v2);
            let rhs = (v3, v4);
            if is_overlapping(lhs, rhs) || is_overlapping(rhs, lhs) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

fn is_overlapping(inside: (i32, i32), outside: (i32, i32)) -> bool {
    (inside.0 >= outside.0 && inside.0 <= outside.1)
        || (inside.1 >= outside.0 && inside.1 <= outside.1)
}
