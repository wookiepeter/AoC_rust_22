#![allow(dead_code)]
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fs,
    rc::{Rc, Weak},
};

use itertools::Itertools;
// Check out: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html to understand
// tree structures in rust!

pub fn run() {
    let input = fs::read_to_string("./assets/input_07.txt").expect("Input file not found!");

    part1and2(&input);
}

fn part1and2(input: &str) {
    let commands = input.split("\n$ ").skip(1);
    let root = Rc::new(Node {
        name: "/".to_string(),
        kind: NodeType::Dir,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let mut active_node = Rc::clone(&root);

    commands.for_each(|command| {
        let mut lines = command.lines();
        let first_line: Vec<&str> = lines.next().unwrap().split(" ").collect();
        match first_line[..] {
            ["ls"] => lines
                .map(|line| line.split_once(" ").unwrap())
                .for_each(|tuple| {
                    match tuple {
                        ("dir", _) => {
                            // create a directory here:
                            let new_node = Rc::new(Node {
                                name: tuple.1.to_string(),
                                kind: NodeType::Dir,
                                parent: RefCell::new(Rc::downgrade(&active_node)),
                                children: RefCell::new(vec![]),
                            });

                            active_node.children.borrow_mut().push(Rc::clone(&new_node));
                        }
                        _ => {
                            // create a file here
                            let new_node = Rc::new(Node {
                                name: tuple.1.to_string(),
                                kind: NodeType::File {
                                    size: tuple.0.parse::<u32>().unwrap(),
                                },
                                parent: RefCell::new(Rc::downgrade(&active_node)),
                                children: RefCell::new(vec![]),
                            });

                            active_node.children.borrow_mut().push(Rc::clone(&new_node));
                        }
                    }
                }),
            ["cd", ".."] => {
                let parent = active_node.as_ref().parent.borrow().upgrade().unwrap();
                active_node = Rc::clone(&parent);
            }
            ["cd", "/"] => {
                active_node = Rc::clone(&root);
            }
            ["cd", _] => {
                let dir_name = first_line[1];
                let future_node = active_node
                    .children
                    .borrow()
                    .iter()
                    .find(|node| node.kind == NodeType::Dir && node.name.eq(dir_name))
                    .expect("Could not find a node!")
                    .clone();

                active_node = Rc::clone(&future_node);
            }
            _ => {
                panic!("Invalid command found: {:?}", first_line);
            }
        }
    });

    println!("Finished building the tree!");

    // Computing size recursively:
    let mut dir_sizes: Vec<u32> = vec![];

    let total_size = root.compute_size(&mut dir_sizes);

    let size = dir_sizes
        .iter()
        .filter(|value| **value <= 100000)
        .sum::<u32>();

    let file_to_delete = dir_sizes
        .iter()
        .sorted()
        .find(|size| total_size - **size <= (70000000 - 30000000))
        .unwrap();

    println!(
        "the total size of directories under 100000 seems to be: {}",
        size
    );
    println!("the total size of all directories is: {}", total_size);
    println!(
        "the directory that should be deleted is: {}",
        file_to_delete
    );
}

#[derive(PartialEq)]
enum NodeType {
    File { size: u32 },
    Dir,
}

struct Node {
    name: String,
    kind: NodeType,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn compute_size(&self, dir_sizes: &mut Vec<u32>) -> u32 {
        match self.kind {
            NodeType::File { size } => {
                return size;
            }
            NodeType::Dir => {
                let size = self
                    .children
                    .borrow()
                    .iter()
                    .map(|node| node.compute_size(dir_sizes))
                    .sum();
                dir_sizes.push(size);
                size
            }
        }
    }
}
