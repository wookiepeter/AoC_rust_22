#![allow(dead_code)]
use std::{fs, ops::Range, vec};

pub fn run() {
    let _test_input = fs::read_to_string("./assets/input_15_test.txt").expect("File not found!");
    let input = fs::read_to_string("./assets/input_15.txt").expect("File not found!");

    let test_sensors = parse_sensors(&_test_input); 

    let test_row = 11; 

    println!("TEST: There is a total of {} positions in row {} where no other beacons can be.",  part1(&test_sensors, test_row), test_row); 

    let test_beacon = part2(&test_sensors, 20); 
    println!("TEST: The distress beacon is at position {:?}, resulting in the frequency {}", test_beacon, test_beacon.0 * 4000000 + test_beacon.1); 
    
    let sensors = parse_sensors(&input); 
    let row = 2000000; 

    println!("There is a total of {} positions in row {} where no other beacons can be.",  part1(&sensors, row), row); 

    let missing_beacon = part2(&sensors, 4000000); 
    println!("The distress beacon is at position {:?}, resulting in the frequency {}", missing_beacon, missing_beacon.0 as i64 * 4000000 + missing_beacon.1 as i64); 
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

        // Check the orthogonal distance from the sensor to the specified row. 
        // If that is smaller than the sensor range we can now use the remaining sensor range 
        // to determine the section of the row in range of this censor (bc. Manhattan distance)
        let remaining_manh_dist: i32 = sensor.radius as i32 - i32::abs(row - sensor.position.1); 
        if remaining_manh_dist > 0 {
            blocked_ranges.push((sensor.position.0 - remaining_manh_dist)..(sensor.position.0 + remaining_manh_dist)); 
        }
    }); 

    blocked_ranges.sort_by_key(|range| range.start); 

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

    joined_ranges.iter().map(|range| range.len() as u32).sum()
}

fn part2(sensors: &Vec<Sensor>, sector_size: i32) -> (i32, i32) {
    let mut blocked_ranges: Vec<Range<i32>> = vec![]; 
    let mut joined_ranges: Vec<Range<i32>> = vec![]; 

    for y in 0..sector_size {
        blocked_ranges.clear(); 
        sensors.iter().for_each(|sensor| {
            let remaining_manh_dist = sensor.radius as i32 - i32::abs(y - sensor.position.1); 
            // Ignore the range if it does not even reach into the sector to avoid having to sort 
            // or remove unneccesary ranges later on
            if remaining_manh_dist > 0 && sensor.position.0 + remaining_manh_dist > 0 && sensor.position.0 - remaining_manh_dist < sector_size {
                blocked_ranges.push((sensor.position.0 - remaining_manh_dist)..(sensor.position.0 + remaining_manh_dist)); 
            }
        }); 

        blocked_ranges.sort_by_key(|range| range.start ); 

        joined_ranges.clear(); 
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

        // more than 1 range left -> the gap between the 2 ranges has to be the beacon!
        if joined_ranges.len() > 1 {
            return (joined_ranges[0].end + 1, y) 
        }
    }

    panic!("Could not find a solution for the Beacon in the given Sector (size: {})", sector_size); 
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
