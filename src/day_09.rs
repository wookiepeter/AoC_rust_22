#![allow(dead_code)]
use std::{fs, vec};

pub fn run() {
    let input = fs::read_to_string("./assets/input_09.txt").expect("File not found!");
    let test_input = fs::read_to_string("./assets/input_09_test.txt").expect("File not found!");

    println!("Total signal strength: {}", part1(&test_input));
    println!("Total signal strength: {}", part1(&input));
}

fn part1(input: &str) -> i32 {
    let num_list: Vec<(i32, i32)> = (1..)
        .zip(
            input
                .lines()
                .map(
                    |command| match command.split_whitespace().collect::<Vec<&str>>()[..] {
                        ["noop"] => vec![0].into_iter(),
                        ["addx", value] => {
                            let arr = vec![0, value.parse::<i32>().unwrap()];
                            arr.into_iter()
                        }
                        _ => {
                            println!("Invalid input!");
                            vec![0].into_iter()
                        }
                    },
                )
                .flatten(),
        )
        .collect();

    let mut acc = 1;
    let mut total_signal_strength = 0;

    for (i, value) in num_list {
        if (i - 20) % 40 == 0 {
            total_signal_strength += (i as i32) * acc;
            println!(
                "during cycle {} the signal strength was {}",
                i,
                i as i32 * acc
            );
        }
        acc += value;
    }

    total_signal_strength
}
