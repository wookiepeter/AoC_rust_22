#![allow(dead_code)]
use std::fs;
use text_io;

// Requires input files to have LF style line endings!
pub fn run() {
    let input = fs::read_to_string("./assets/input_05.txt").expect("Could not load file!");

    part1(&input);
}

fn part1(input: &str) {
    let mut stacks = Stacks::new(input);
    stacks.print_stacks();

    let (_, move_input) = input.split_once("\n\n").expect("Invalid input!");
    move_input.lines().for_each(|line| {
        let (num, from, to): (usize, usize, usize);
        text_io::scan!(line.bytes() => "move {} from {} to {}", num, from, to);
        stacks.move_crates(num, from - 1, to - 1);
    });

    println!("Top stack: {}", stacks.top_stacks());
}

fn part2(input: &str) {
    let mut stacks = Stacks::new(input);
    stacks.print_stacks();

    let (_, move_input) = input.split_once("\n\n").expect("Invalid input!");
    move_input.lines().for_each(|line| {
        let (num, from, to): (usize, usize, usize);
        text_io::scan!(line.bytes() => "move {} from {} to {}", num, from, to);
        stacks.move_stack(num, from - 1, to - 1);
    });

    println!("Top stack: {}", stacks.top_stacks());
}

struct Stacks {
    array: Vec<Vec<char>>,
}

impl Stacks {
    fn new(input: &str) -> Stacks {
        let (map_input, _) = input.split_once("\n\n").expect("Invalid Input!");

        let (first_line, _) = map_input.split_once("\n").unwrap();
        let mut array: Vec<Vec<char>> = vec![vec![]; (first_line.len() + 1) / 4];

        map_input.lines().rev().skip(1).for_each(|line| {
            let line_vec: Vec<char> = line.chars().collect();
            for i in 0..array.len() {
                let next_char = line_vec[1 + i * 4];
                match next_char {
                    ' ' => {}
                    value => {
                        array[i].push(value);
                    }
                }
            }
        });

        Stacks { array }
    }

    fn move_stack(&mut self, num: usize, from: usize, to: usize) {
        let stack_range = (self.array[from].len() - num)..(self.array[from].len());
        let mut arr: Vec<char>;
        arr = self
            .array
            .get_mut(from)
            .unwrap()
            .drain(stack_range)
            .rev()
            .collect();

        self.array.get_mut(to).unwrap().append(&mut arr)
    }

    fn move_crates(&mut self, num: usize, from: usize, to: usize) {
        let stack_range = (self.array[from].len() - num)..(self.array[from].len());
        let mut arr: Vec<char>;
        arr = self
            .array
            .get_mut(from)
            .unwrap()
            .drain(stack_range)
            .collect();

        self.array.get_mut(to).unwrap().append(&mut arr)
    }

    fn top_stacks(&self) -> String {
        self.array
            .iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect::<String>()
    }

    fn print_stacks(&self) {
        println!("-----");
        self.array.iter().for_each(|stack| {
            println!("{:?}", stack);
        });
        println!("-----");
    }
}
