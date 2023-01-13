#![allow(dead_code)]
use std::{fmt::Display, fs, iter};

pub fn run() {
    let input = fs::read_to_string("./assets/input_14.txt").expect("File not found!");
    let _test_input = fs::read_to_string("./assets/input_14_test.txt").expect("File not found!");

    let (mut map, highest_y) = build_map(&input);

    println!("Highest y-value: {}", highest_y);
    let sand_on_structures = part1(&mut map, highest_y);

    println!("Total sand: {}", sand_on_structures);

    add_lower_wall_to_map(&mut map, highest_y + 2);

    let til_blockage = part2(&mut map);
    map.print_map();

    println!(
        "Total sand until blockage: {}",
        sand_on_structures + til_blockage
    );
}

fn build_map(input: &str) -> (Map<char>, usize) {
    let mut map = Map::new((1000, 200), '.');
    let mut highest_y: usize = 0;

    input.lines().for_each(|line| {
        let wall_coords: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|corner| {
                let (v1, v2) = corner.split_once(',').unwrap();
                (v1.parse::<usize>().unwrap(), v2.parse::<usize>().unwrap())
            })
            .collect();

        let mut corner_from = wall_coords[0];
        highest_y = usize::max(corner_from.1, highest_y);
        for i in 1..wall_coords.len() {
            let corner_to = wall_coords[i];
            highest_y = usize::max(corner_to.1, highest_y);

            let lane: Vec<(usize, usize)> = match corner_from.0 == corner_to.0 {
                true => iter::repeat(corner_from.0)
                    .zip(
                        usize::min(corner_from.1, corner_to.1)
                            ..=usize::max(corner_from.1, corner_to.1),
                    )
                    .collect(),
                false => (usize::min(corner_from.0, corner_to.0)
                    ..=usize::max(corner_from.0, corner_to.0))
                    .zip(iter::repeat(corner_from.1))
                    .collect(),
            };

            lane.iter().for_each(|pos| {
                map.set(pos, '#');
            });
            corner_from = corner_to;
        }
    });

    (map, highest_y)
}

fn part1(map: &mut Map<char>, max_y: usize) -> u32 {
    for sand in 0..(map.map_size.0 * map.map_size.1) {
        let mut cur_pos = (500, 0);
        loop {
            match map.check_below(cur_pos) {
                Some(value) => {
                    cur_pos = value;
                    if value.1 > max_y {
                        return sand as u32;
                    }
                }
                None => {
                    map.set(&cur_pos, 'o');
                    break;
                }
            }
        }
    }
    return 0;
}

fn add_lower_wall_to_map(map: &mut Map<char>, highest_y: usize) {
    map.array[highest_y] = vec!['#'; map.map_size.0];
}

/// This function fills up the existing map -> therefore if you hand it a map that was already
/// modified by fn part1(...) it will only return the remaining sand needed to block the input
fn part2(map: &mut Map<char>) -> u32 {
    for sand in 0..(map.map_size.0 * map.map_size.1) {
        let mut cur_pos = (500, 0);
        loop {
            match map.check_below(cur_pos) {
                Some(value) => {
                    cur_pos = value;
                }
                None => {
                    if cur_pos == (500, 0) {
                        return sand as u32 + 1;
                    }
                    map.set(&cur_pos, 'o');
                    break;
                }
            }
        }
    }
    return 0;
}

struct Map<T> {
    array: Vec<Vec<T>>,
    map_size: (usize, usize),
}

impl<T: Clone + Copy + Display + PartialEq> Map<T> {
    fn new(size: (usize, usize), default_value: T) -> Map<T> {
        Map {
            array: vec![vec![default_value; size.0]; size.1],
            map_size: size,
        }
    }

    fn is_inbounds(&self, coords: &(usize, usize)) -> bool {
        coords.0 < self.map_size.0 && coords.1 < self.map_size.0
    }

    fn get(&self, coords: &(usize, usize)) -> Option<T> {
        match self.is_inbounds(coords) {
            true => Some(self.array[coords.1][coords.0]),
            false => None,
        }
    }

    fn get_unsafe(&self, coords: &(usize, usize)) -> T {
        self.array[coords.1][coords.0]
    }

    fn set(&mut self, coords: &(usize, usize), value: T) -> bool {
        if !self.is_inbounds(coords) {
            return false;
        }
        self.array[coords.1][coords.0] = value;
        true
    }
}

impl Map<char> {
    fn print_map(&self) {
        self.array
            .iter()
            .for_each(|line| println!("{}", String::from_iter(line[400..600].iter())));
    }

    /// returns An option with the next position according to the the algorithm
    /// -> down; down left; down right;
    /// if those are not free -> return None
    fn check_below(&self, coords: (usize, usize)) -> Option<(usize, usize)> {
        if self.get_unsafe(&(coords.0, coords.1 + 1)) == '.' {
            return Some((coords.0, coords.1 + 1));
        }
        if self.get_unsafe(&(coords.0 - 1, coords.1 + 1)) == '.' {
            return Some((coords.0 - 1, coords.1 + 1));
        }
        if self.get_unsafe(&(coords.0 + 1, coords.1 + 1)) == '.' {
            return Some((coords.0 + 1, coords.1 + 1));
        }
        None
    }
}
