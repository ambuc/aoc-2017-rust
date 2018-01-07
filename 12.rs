use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

fn clean_and_parse_u32(inp: &str) -> u32 {
    inp.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

struct Graph {
    adj: Vec<(u32, u32)>,
    nodes: HashSet<u32>,
}

impl Graph {
    fn new(s: String) -> Graph {
        let mut g: Graph = Graph {
            adj: Vec::new(),
            nodes: HashSet::new(),
        };
        for line in s.lines() {
            let mut iter = line.split_whitespace();
            let from: u32 = clean_and_parse_u32(iter.next().unwrap());
            g.nodes.insert(from);
            let _ = iter.next();
            for to in iter {
                g.adj.push((from, clean_and_parse_u32(to)));
            }
        }
        g
    }

    fn adj_to(&self, node: u32) -> Vec<u32> {
        let mut siblings: Vec<u32> = Vec::new();
        for pair in &self.adj {
            if pair.0 == node {
                siblings.push(pair.1)
            }
        }
        siblings
    }

    fn component_with(&self, node: u32) -> HashSet<u32> {
        let mut group: HashSet<u32> = HashSet::new();
        let mut queue: Vec<u32> = vec![node];
        while !queue.is_empty() {
            let curr: u32 = queue.pop().unwrap();
            group.insert(curr);
            for sib in &self.adj_to(curr) {
                if !group.contains(sib) {
                    queue.push(*sib);
                }
            }
        }
        group
    }

    fn components(&self) -> Vec<HashSet<u32>> {
        let mut groups: Vec<HashSet<u32>> = vec![];
        let mut queue: Vec<u32> = Vec::new();
        for node in &self.nodes {
            queue.push(*node);
        }
        let mut seen: HashSet<u32> = HashSet::new();

        while !queue.is_empty() {
            let curr: u32 = queue.pop().unwrap();
            if seen.contains(&curr) {
                continue;
            }
            let curr_group = &self.component_with(curr);
            for node in curr_group {
                seen.insert(*node);
            }
            groups.push(curr_group.clone());
        }
        groups
    }
}

fn main() {
    let path = Path::new("input/12.txt");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let graph: Graph = Graph::new(s);

    println!("{}", String::from("2017 AOC #12"));
    println!("Part One: {:?}", graph.component_with(0).len());
    println!("Part Two: {:?}", graph.components().len());
}
