#![allow(dead_code)]
use std::{fs, ops::Range};

use itertools::Itertools;

pub fn run() {
    let _test_input = fs::read_to_string("./assets/input_12_test.txt").expect(" File not found!");
    let input = fs::read_to_string("./assets/input_12.txt").expect(" File not found!");

    let mut start_position: (usize, usize) = (0, 0);
    let mut end_position: (usize, usize) = (0, 0);

    let height_map: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start_position = (x, y);
                        0
                    }
                    'E' => {
                        end_position = (x, y);
                        25
                    }
                    other => other as usize - 'a' as usize,
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    part1(&end_position, &height_map, &input, &start_position);
    part2(&end_position, &height_map, &input);
}

fn part1(
    end_position: &(usize, usize),
    height_map: &Vec<Vec<usize>>,
    input: &str,
    start_position: &(usize, usize),
) {
    let mut check_positions: Vec<(usize, usize)> = vec![end_position.clone()];
    let map_size = (height_map[0].len(), height_map.len());
    let mut visited_map: Vec<Vec<bool>> = vec![vec![false; map_size.0]; map_size.1];
    let mut debug_map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut rounds_required = 0;
    for i in 1.. {
        let next_checks: Vec<(usize, usize)> = check_positions
            .iter()
            .flat_map(|pos| {
                let pos_height = height_map[pos.1][pos.0];

                let neighbors: Vec<(usize, usize)> = pos
                    .get_neighbors(&map_size)
                    .iter()
                    .filter_map(|neighbor| {
                        if visited_map[neighbor.1][neighbor.0] == true {
                            return None;
                        }
                        if height_map[neighbor.1][neighbor.0] < pos_height
                            && pos_height - height_map[neighbor.1][neighbor.0] > 1
                        {
                            return None;
                        }
                        Some(*neighbor)
                    })
                    .collect();

                neighbors.into_iter()
            })
            .sorted()
            .dedup()
            .collect();

        if let Some(_) = next_checks.iter().find(|pos| **pos == *start_position) {
            rounds_required = i;
            break;
        }
        next_checks.iter().for_each(|pos| {
            visited_map[pos.1][pos.0] = true;
            debug_map[pos.1][pos.0] =
                std::char::from_u32('A' as u32 + height_map[pos.1][pos.0] as u32).unwrap();
        });

        check_positions = next_checks;
    }
    println!(
        "Found a way to the target position after {} steps",
        rounds_required
    );
}

fn part2(end_position: &(usize, usize), height_map: &Vec<Vec<usize>>, input: &str) {
    let mut check_positions: Vec<(usize, usize)> = vec![end_position.clone()];
    let map_size = (height_map[0].len(), height_map.len());
    let mut visited_map: Vec<Vec<bool>> = vec![vec![false; map_size.0]; map_size.1];
    let mut debug_map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut rounds_required = 0;
    for i in 1.. {
        let next_checks: Vec<(usize, usize)> = check_positions
            .iter()
            .flat_map(|pos| {
                let pos_height = height_map[pos.1][pos.0];

                let neighbors: Vec<(usize, usize)> = pos
                    .get_neighbors(&map_size)
                    .iter()
                    .filter_map(|neighbor| {
                        if visited_map[neighbor.1][neighbor.0] == true {
                            return None;
                        }
                        if height_map[neighbor.1][neighbor.0] < pos_height
                            && pos_height - height_map[neighbor.1][neighbor.0] > 1
                        {
                            return None;
                        }
                        Some(*neighbor)
                    })
                    .collect();

                neighbors.into_iter()
            })
            .sorted()
            .dedup()
            .collect();

        if let Some(_) = next_checks.iter().find(|pos| height_map[pos.1][pos.0] == 0) {
            rounds_required = i;
            break;
        }
        next_checks.iter().for_each(|pos| {
            visited_map[pos.1][pos.0] = true;
            debug_map[pos.1][pos.0] =
                std::char::from_u32('A' as u32 + height_map[pos.1][pos.0] as u32).unwrap();
        });

        check_positions = next_checks;
    }
    println!(
        "Found a way to the target position after {} steps",
        rounds_required
    );
}

fn print_debug_map(debug_map: &Vec<Vec<char>>) {
    debug_map.iter().for_each(|line| {
        let s: String = line.iter().collect();
        println!("{}", s);
    })
}

fn print_debug_map_region(debug_map: &Vec<Vec<char>>, region: (Range<usize>, Range<usize>)) {
    for y in region.1 {
        let line: String = debug_map[y][region.0.clone()].iter().collect();
        println!("{}", line);
    }
    println!("------------------");
}

trait TupleCoords<T> {
    fn get_neighbors(&self, map_size: &T) -> Vec<T>;
}

impl TupleCoords<(usize, usize)> for (usize, usize) {
    fn get_neighbors(&self, map_size: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];
        if self.0 > 0 {
            result.push((self.0 - 1, self.1));
        }
        if self.1 > 0 {
            result.push((self.0, self.1 - 1));
        }
        if self.0 < map_size.0 - 1 {
            result.push((self.0 + 1, self.1));
        }
        if self.1 < map_size.1 - 1 {
            result.push((self.0, self.1 + 1));
        }
        result
    }
}
