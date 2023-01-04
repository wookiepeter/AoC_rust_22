#![allow(dead_code)]
use std::{collections::HashSet, fs};

// TODO: look at other solutions for a better strategy!
pub fn run() {
    let input = fs::read_to_string("./assets/input_08.txt").expect("File not found!");

    let ew_lanes: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut visible_map: HashSet<(usize, usize)> = HashSet::new();

    let mut cur_height: i32;

    // east to west
    for y in 1..(ew_lanes.len() - 1) {
        cur_height = -1;
        for x in 0..ew_lanes[0].len() {
            if ew_lanes[y][x] > cur_height {
                visible_map.insert((x, y));
                cur_height = ew_lanes[y][x];
                if cur_height >= 9 {
                    break;
                }
            }
        }
    }

    // west to east
    for y in 1..(ew_lanes.len() - 1) {
        cur_height = -1;
        for x in (0..ew_lanes[0].len()).rev() {
            if ew_lanes[y][x] > cur_height {
                visible_map.insert((x, y));
                cur_height = ew_lanes[y][x];
                if cur_height >= 9 {
                    break;
                }
            }
        }
    }

    // north to south
    for x in 1..(ew_lanes[0].len() - 1) {
        cur_height = -1;
        for y in 0..ew_lanes.len() {
            if ew_lanes[y][x] > cur_height {
                visible_map.insert((x, y));
                cur_height = ew_lanes[y][x];
                if cur_height >= 9 {
                    break;
                }
            }
        }
    }

    // south to north
    for x in 1..(ew_lanes[0].len() - 1) {
        cur_height = -1;
        for y in (0..ew_lanes.len()).rev() {
            if ew_lanes[y][x] > cur_height {
                visible_map.insert((x, y));
                cur_height = ew_lanes[y][x];
                if cur_height >= 9 {
                    break;
                }
            }
        }
    }

    visible_map.insert((0, 0));
    visible_map.insert((0, ew_lanes.len()));
    visible_map.insert((ew_lanes.len(), 0));
    visible_map.insert((ew_lanes.len(), ew_lanes.len()));

    println!("number of elements: {}", visible_map.len());

    let mut cur_max_view = 0;

    for y in 1..(ew_lanes.len() - 1) {
        for x in 1..(ew_lanes[0].len() - 1) {
            let mut view_score = 1;
            let cur_tree_height = ew_lanes[y][x];
            // check west
            match (1..x).find(|cur| ew_lanes[y][x - cur] >= cur_tree_height) {
                None => view_score *= x,
                Some(value) => view_score *= value,
            }
            // check east
            let dist_to_border = ew_lanes[0].len() - 1 - x;
            match (1..dist_to_border).find(|cur| ew_lanes[y][x + cur] >= cur_tree_height) {
                None => view_score *= dist_to_border,
                Some(value) => view_score *= value,
            }

            // check north
            match (1..y).find(|cur| ew_lanes[y - cur][x] >= cur_tree_height) {
                None => view_score *= y,
                Some(value) => view_score *= value,
            }

            // check south
            let dist_to_border = ew_lanes.len() - 1 - y;
            match (1..dist_to_border).find(|cur| ew_lanes[y + cur][x] >= cur_tree_height) {
                None => view_score *= dist_to_border,
                Some(value) => view_score *= value,
            }

            // update score if necessary
            if cur_max_view <= view_score {
                println!(
                    "found new max view score ({}) at position ({},{})",
                    view_score, x, y
                );
                cur_max_view = view_score;
            }
        }
    }

    println!("highest found viewscore was {}", cur_max_view);
}
