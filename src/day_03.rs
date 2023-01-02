#![allow(dead_code)]
use std::fs;

pub fn run() {
    let input_data =
        fs::read_to_string("./assets/input_03.txt").expect("Input file did not exist!");

    println!("priority score was: {}", part1(&input_data));
    println!("three elf item score was: {}", part2(&input_data));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let indices: Vec<usize> = line
                .chars()
                .map(|letter| {
                    let mut index = letter as usize;
                    if index >= ('a' as usize) {
                        index -= 'a' as usize
                    } else {
                        index = index - ('A' as usize) + 26
                    }
                    index
                })
                .collect();

            let (bag1, bag2) = indices.split_at(line.len() / 2);
            let mut used_letters = vec![false; 52];

            for index in bag1 {
                used_letters[*index] = true;
            }
            for index in bag2 {
                if used_letters[*index] == true {
                    return (*index + 1) as u32;
                }
            }
            0
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut lines = input.lines();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let mut combined = pars_chars(line.unwrap());
        combined.and(&pars_chars(lines.next().unwrap()));
        combined.and(&pars_chars(lines.next().unwrap()));
        for i in 0..combined.len() {
            if combined.get(i).unwrap() {
                sum += i + 1;
                break;
            }
        }
    }
    sum as u32
}

fn pars_chars(line: &str) -> bit_vec::BitVec {
    let mut result = bit_vec::BitVec::from_elem(52, false);
    for letter in line.chars() {
        let mut index = letter as usize;
        if index >= ('a' as usize) {
            index -= 'a' as usize
        } else {
            index = index - ('A' as usize) + 26
        }
        result.set(index, true);
    }
    return result;
}
