#![allow(dead_code)]
use std::fs::{self};

pub fn run() {
    let data = fs::read_to_string("./assets/input_01.txt").expect("File didn't load properly.");

    println!("Max Calories: {:?}", part1(&data));

    println!("Top 3 Calories: {}", part2(&data));
}

fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let reindeer = input.split("\n\n").map(|calories| {
        calories
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>()
    });

    itertools::sorted(reindeer).rev().take(3).sum()
}
