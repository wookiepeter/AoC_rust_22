#![allow(dead_code)]
use std::fs;

pub fn run() {
    let input = fs::read_to_string("./assets/input_06.txt").expect("Missing input file!");

    println!("first signal was found at: {}", solve_with_loop(&input, 4));
    println!(
        "Second signal was found at: {}",
        solve_with_loop(&input, 14)
    );
    println!("first signal was found at {}", solve_with_iter(&input, 4));
    println!(
        "Second signal was found at: {}",
        solve_with_iter(&input, 14)
    );
}

/// This is probably a decent solution in regards to performance, but it's not very clean
fn solve_with_loop(input: &str, signal_len: usize) -> usize {
    let input_vec: Vec<char> = input.chars().collect();
    let mut unique_counter: usize = 0;
    let mut should_reset = false;
    let mut resulting_index: usize = 0;

    // this would crash if the input does not have the 'start-of-packet-marker'
    for i in 0..input_vec.len() {
        for j in 1..(signal_len - unique_counter) {
            if input_vec[i] == input_vec[i + j] {
                should_reset = true;
                break;
            }
        }
        if should_reset {
            should_reset = false;
            unique_counter = 0;
            continue;
        } else {
            unique_counter += 1;
            if unique_counter == signal_len {
                resulting_index = i + 1; // resulting index should include the marker itself!
                break;
            }
        }
    }

    resulting_index
}

fn solve_with_iter(input: &str, signal_len: usize) -> usize {
    let input_vec = input.chars().collect::<Vec<char>>();

    let sequence = input_vec
        .windows(signal_len)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<std::collections::HashSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();

    sequence.0 + signal_len
}
