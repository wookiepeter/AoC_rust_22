#![allow(dead_code)]
use std::{cmp::Ordering, fs};

use itertools::Itertools;

pub fn run() {
    let _test_input = fs::read_to_string("./assets/input_13_test.txt").expect("File not found!");
    let input = fs::read_to_string("./assets/input_13.txt").expect("File not found!");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let pairs: Vec<&str> = input.split("\n\n").collect();
    let correctly_ordered: i32 = pairs
        .into_iter()
        .enumerate()
        .map(|(i, pair)| {
            let first_node = Node::from(pair.lines().next().unwrap());
            let second_node = Node::from(pair.lines().skip(1).next().unwrap());

            let order = are_nodes_ordered(&first_node, &second_node);
            match order {
                Ordering::Less => (i as i32) + 1,
                _ => 0,
            }
        })
        .sum();
    println!("Correctly ordered pairs: {}", correctly_ordered);
}

fn part2(input: &str) {
    let mut input_str = String::from(input);
    input_str.push_str("\n[[2]]\n[[6]]\n");

    let node_list: Vec<Node> = input_str
        .split("\n\n")
        .flat_map(|pair| pair.lines())
        .map(|line| Node::from(line))
        .sorted_by(|a, b| are_nodes_ordered(a, b))
        .collect();

    let first_item = Node::List(vec![Node::List(vec![Node::Number(2)])]);
    let (first_index, _) = node_list
        .iter()
        .find_position(|node| are_nodes_ordered(node, &first_item) == Ordering::Equal)
        .unwrap();

    let second_item = Node::List(vec![Node::List(vec![Node::Number(6)])]);
    let (second_index, _) = node_list
        .iter()
        .find_position(|node| are_nodes_ordered(node, &second_item) == Ordering::Equal)
        .unwrap();

    println!(
        "Found the combined positions: {}",
        (first_index + 1) * (second_index + 1)
    );
}

#[derive(Debug, Clone)]
enum Node {
    Number(i32),
    List(Vec<Node>),
}

impl Node {
    fn from(content: &str) -> Node {
        let mut result: Vec<Node> = vec![];
        let mut current_index = 0;
        loop {
            /*
            println!(
                "currently parsing: {} - starting at element: {}",
                content, current_index
            );
            */
            if current_index >= content.len() {
                break;
            }
            match content[current_index..].chars().next().unwrap() {
                '[' => {
                    let matching_bracket = find_matching_bracket(&content[current_index..]);
                    result.push(Node::from(
                        &content[(current_index + 1)..=(current_index + matching_bracket)],
                    ));
                    current_index = current_index + matching_bracket + 2;
                }
                ',' => {
                    current_index += 1;
                    // println!("Should have skipped a comma");
                }
                _ => match content[current_index..]
                    .chars()
                    .find_position(|c| *c == '[' || *c == ',')
                {
                    Some((end_index, _some_char)) => {
                        result.push(Node::Number(
                            content[current_index..(current_index + end_index)]
                                .parse::<i32>()
                                .unwrap(),
                        ));
                        current_index = current_index + end_index;
                    }
                    None => {
                        if current_index == 0 {
                            return Node::Number(content.parse::<i32>().unwrap());
                        } else {
                            result.push(Node::Number(
                                content[current_index..].parse::<i32>().unwrap(),
                            ));
                            current_index += 1;
                        }
                    }
                },
            }
        }
        return Node::List(result);
    }
}

fn find_matching_bracket(content: &str) -> usize {
    let mut bracket_count = 1;
    for (i, c) in content.chars().skip(1).enumerate() {
        match c {
            '[' => bracket_count += 1,
            ']' => bracket_count -= 1,
            _ => (),
        }

        if bracket_count == 0 {
            return i;
        }
    }
    panic!["Should have found a matching bracket in: {}", content];
}

fn are_nodes_ordered(lhs: &Node, rhs: &Node) -> Ordering {
    let result: Ordering = match (lhs, rhs) {
        (Node::Number(left_value), Node::Number(right_value)) => {
            i32::cmp(&left_value, &right_value)
        }
        (Node::List(left_list), Node::List(right_list)) => {
            let combined_iter = left_list.iter().zip(right_list.iter());

            for pair in combined_iter {
                let order = are_nodes_ordered(&pair.0, &pair.1);
                if order != Ordering::Equal {
                    return order;
                }
            }
            i32::cmp(&(left_list.len() as i32), &(right_list.len() as i32))
        }
        (Node::List(left_value), Node::Number(right_value)) => are_nodes_ordered(
            &Node::List(left_value.clone()),
            &Node::List(vec![Node::Number(right_value.clone())]),
        ),
        (Node::Number(left_value), Node::List(right_value)) => are_nodes_ordered(
            &Node::List(vec![Node::Number(left_value.clone())]),
            &Node::List(right_value.clone()),
        ),
    };

    result
}
