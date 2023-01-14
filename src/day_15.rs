#![allow(dead_code)]
use std::{fs, ops::Range};

pub fn run() {
    let _test_input = fs::read_to_string("./assets/input_15_test.txt").expect("File not found!");
    let input = fs::read_to_string("./assets/input_15.txt").expect("File not found!");

    let test_sensors = parse_sensors(&_test_input); 

    let test_row = 11; 

    println!("TEST: There is a total of {} positions in row {} where no other beacons can be.",  part1(&test_sensors, test_row), test_row); 
    
    let sensors = parse_sensors(&input); 
    let row = 2000000; 

    println!("There is a total of {} positions in row {} where no other beacons can be.",  part1(&sensors, row), row); 
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input.lines().map(|line| {
        let (sx, sy, bx, by): (i32, i32, i32, i32); 
        text_io::scan!(line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by); 
        Sensor{
            position: (sx, sy), 
            radius: (sx, sy).manh_distance((bx, by)), 
        }
    }).collect::<Vec<Sensor>>()
}

fn part1(sensors: &Vec<Sensor>, row: i32) -> u32 {
    let mut blocked_ranges: Vec<Range<i32>> = vec![]; 

    sensors.iter().for_each(|sensor| {
        let remaining_manh_dist: i32 = sensor.radius as i32 - i32::abs(row - sensor.position.1); 
        if remaining_manh_dist > 0 {
            blocked_ranges.push((sensor.position.0 - remaining_manh_dist)..(sensor.position.0 + remaining_manh_dist)); 
            // println!("Found range {:?} for {:?}", blocked_ranges[blocked_ranges.len() - 1 as usize], sensor); 
        }
        else {
            // println!("Found no range for {:?}", sensor); 
        }
    }); 

    blocked_ranges.sort_by_key(|range| range.start); 

    // blocked_ranges.iter().for_each(|range| println!("{:?}", range)); 

    let mut joined_ranges: Vec<Range<i32>> = vec![]; 
    let mut current_range = blocked_ranges[0].clone(); 

    for i in  1..blocked_ranges.len() {
        if current_range.end < blocked_ranges[i].start {
            joined_ranges.push(current_range); 
            current_range = blocked_ranges[i].clone(); 
            continue;
        }
        current_range = current_range.start..current_range.end.max(blocked_ranges[i].end); 
    }
    // add in the last remaining element
    joined_ranges.push(current_range);
    // println!("have {} joined ranges", joined_ranges.len()); 
    // joined_ranges.iter().for_each(|range| println!("{:?}", range)); 

    joined_ranges.iter().map(|range| range.len() as u32).sum()
}

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    radius: u32,
}

trait ManhattanDistance<T> {
    fn manh_distance(&self, other: T) -> u32;
}

impl ManhattanDistance<(i32, i32)> for (i32, i32) {
    fn manh_distance(&self, other: (i32, i32)) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}
