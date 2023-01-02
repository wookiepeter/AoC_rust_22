#![allow(dead_code)]
use std::fs;

pub fn run() {
    let input_string = fs::read_to_string("./assets/input_02.txt").expect("File not found!");

    println!("total score: {}", part1(&input_string));

    println!("part2 total score: {}", part2(&input_string));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let tuple = (
                chars[0] as u32 - 'A' as u32 + 1,
                chars[2] as u32 - 'X' as u32 + 1,
            );
            match tuple {
                (1, 1) | (2, 2) | (3, 3) => 3 + tuple.1,
                (1, 2) | (2, 3) | (3, 1) => 6 + tuple.1,
                _ => tuple.1,
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let tuple = (chars[0] as i32 - 'A' as i32, chars[2]);

            let score = match tuple.1 {
                'Y' => 3 + tuple.0 + 1,
                'X' => match tuple.0 {
                    1 | 2 => (tuple.0 - 1) + 1,
                    _ => 3,
                },
                'Z' => 6 + ((tuple.0 + 1) % 3) + 1,
                _ => 0,
            };
            score
        })
        .sum::<i32>() as u32
}
