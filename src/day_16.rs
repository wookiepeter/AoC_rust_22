#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

pub fn run() {
    let _test_input = fs::read_to_string("./assets/input_16_test.txt").expect("File not found!");
    let input = fs::read_to_string("./assets/input_16.txt").expect("File not found!");

    let test_nodes = parse_graph(&_test_input); 
    let test_graph = simplify_graph(&test_nodes);  
    // println!("TEST-GRAPH:"); 
    // test_graph.iter().for_each(|node| println!("{:?}", node.1)); 

    let graph = simplify_graph(&parse_graph(&input)); 
    println!("GRAPH:"); 
    graph.iter().for_each(|node| println!("{:?}", node.1)); 

}

fn parse_graph(input: &str) -> HashMap<String, Node> {
    HashMap::from_iter(
        input.lines().map(|line| {
            let name: String;
            let flow_rate: String;
            let node_string: String;

            let modified_line = line.replace("tunnel leads to valve", "tunnels lead to valves"); 
            let modified_line = String::from(format!("{}.", modified_line)); 
            text_io::scan!(modified_line.bytes() => "Valve {} has flow rate={}; tunnels lead to valves {}.", name, flow_rate, node_string); 

            println!("parsed some line"); 
            let edges = node_string.split(", ").map(|_name| (String::from(_name), 1)).collect::<Vec<(String, usize)>>(); 
            (name.clone(), 
            Node {
                name,
                flow_rate: flow_rate.parse::<u32>().unwrap(),
                edges,
            })
        }
    ))
}

/// Should simplify the graph and remove all Non
fn simplify_graph(graph: &HashMap<String, Node>) -> HashMap<String, Node> {
    let mut result: HashMap<String, Node> = HashMap::new(); 
    let relevant_nodes: Vec<Node> = graph
        .into_iter()
        .filter(|(_, node)| node.flow_rate > 0 || node.name.eq(&"AA"))
        .map(|(_, node)| Node {
            name: node.name.clone(),
            flow_rate: node.flow_rate,
            edges: vec![],
        })
        .collect();

    for i in 0..(relevant_nodes.len()) {
        let node = &relevant_nodes[i]; 
        result.insert(node.name.clone(), Node { 
            name: node.name.clone(),  
            flow_rate: node.flow_rate,   
            edges: find_all_edges(graph, &(relevant_nodes[i].name))
        }); 
    }

    result
}

fn find_all_edges(graph: &HashMap<String, Node>, start: &String) -> Vec<(String, usize)>{
    let mut result: Vec<(String, usize)> = vec![]; 
    let mut visited_nodes: HashSet<String> = HashSet::new();
    visited_nodes.insert(start.clone()); 
    let mut current_edges = graph.get(start).unwrap().edges.clone();

    for edge_count in 1.. {
        // no edges remain to explore 
        if current_edges.len() <= 0 {
            break; 
        }
        // insert connections to relevant endpoints (flowrate > 0) into result
        let mut new_connections = current_edges.iter()
            .filter_map(|(node_key, _)| 
                match graph.get(node_key).unwrap().flow_rate > 0 || node_key.eq(&"AA"){
                    true => Some((node_key.clone(), edge_count)), 
                    false => None,
                })
            .unique()
            .collect::<Vec<(String, usize)>>(); 
        result.append(&mut new_connections); 

        current_edges.iter().for_each(|(node_key, _)| {visited_nodes.insert(node_key.clone()); }); 
        current_edges = current_edges.iter().flat_map(|(node_key, _)| {
            graph.get(node_key).unwrap().edges.clone().into_iter()
        }).filter(|(node_key, _)| !visited_nodes.contains(node_key)).collect(); 
    }

    result.into_iter().filter(|(name, _)| !name.eq("AA")).collect()
}

fn part1(input: &str) -> u32 {
    let nodes = parse_graph(input);

    0
}

/// Fields: 
/// - the value of the Search-Tree so far 
/// - some struct conveying what nodes are still available or already blocked
/// - 
struct STNode {

}

#[derive(Debug)]
struct Node {
    name: String,
    flow_rate: u32,
    edges: Vec<(String, usize)>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

