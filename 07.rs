use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

struct Graph<'a> {
    asc_list: Vec<(&'a str, &'a str)>, //Vec::new();
    masses: HashMap<&'a str, u32>,     //HashMap::new();
    root_node: Option<&'a str>,
}

impl<'a> Graph<'a> {
    fn find_root_node(&mut self) {
        let mut keys: HashSet<&str> = HashSet::new(); //create set of keys, set of values
        let mut vals: HashSet<&str> = HashSet::new(); //root will be a key but not a value
                                                      //so take the set difference
        for (from, to) in self.asc_list.clone() {
            keys.insert(from);
            vals.insert(to);
        }

        self.root_node = Some(keys.difference(&vals).next().unwrap());
    }
    fn root_node(&self) -> &str {
        self.root_node.unwrap()
    }

    fn kids_of(&self, node: &str) -> Vec<&str> {
        let mut kids: Vec<&str> = Vec::new();
        for &(a, bs) in &self.asc_list {
            if a == node {
                kids.push(bs);
            }
        }
        return kids;
    }

    fn mass_of(&self, node: &str) -> u32 {
        *self.masses.get(node).unwrap()
    }

    fn mass_above(&self, node: &str) -> u32 {
        let mut sum: u32 = 0;
        for kid in &self.kids_of(node) {
            sum += &self.mass_above_inclusive(kid);
        }
        return sum;
    }
    fn mass_above_inclusive(&self, node: &str) -> u32 {
        let mut sum: u32 = 0;
        sum += &self.mass_of(node);
        sum += &self.mass_above(node);
        return sum;
    }
    fn find_error(&self) -> u32 {
        return self.find_error_within_subtree_at(&self.root_node());
    }
    fn find_error_within_subtree_at(&self, node: &str) -> u32 {
        let mut list: Vec<(&str, u32)> = Vec::new();
        for kid in &self.kids_of(node) {
            list.push((kid, self.mass_above_inclusive(kid)));
        }
        println!("{} -> {:?}", node, list);
        match try_find_key_for_uniq_val(list) {
            Some(subnode) => return self.find_error_within_subtree_at(subnode),
            None => return 0,
        }
    }
}

fn try_find_key_for_uniq_val(list: Vec<(&str, u32)>) -> Option<&str> {
    // guaranteed list of at least three
    let expected: u32;
    if list[0].1 == list[1].1 {
        expected = list[0].1;
        for &(key, val) in list.iter() {
            if val != expected {
                return Some(key);
            }
        }
        return None;
    } else if list[0].1 == list[2].1 {
        return Some(list[1].0);
    } else if list[1].1 == list[2].1 {
        return Some(list[0].0);
    }
    return None;
}

fn main() {
    let path = Path::new("input/07.txt");
    //let path = Path::new("input/07-test.txt");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    let mut graph: Graph = Graph {
        asc_list: Vec::new(),
        masses: HashMap::new(),
        root_node: None,
    };

    file.read_to_string(&mut s).unwrap();

    for line in s.lines() {
        let mut it = line.split_whitespace();
        let node: &str = it.next().unwrap();
        let mut mass_str: &str = it.next().unwrap();
        // strip surrounding parentheses
        mass_str = &mass_str[1..mass_str.len() - 1];
        // and insert into masses hashmap
        let mass: u32 = mass_str.parse::<u32>().unwrap();
        graph.masses.insert(node, mass);
        // then push (node,child) pairs into asc_list
        for child in &mut it.skip(1) {
            // if the node string ends with a comma, don't write the comma
            let child_ref: &str = if child.contains(",") {
                &child[..child.len() - 1]
            } else {
                child
            };
            graph.asc_list.push((node, child_ref));
        }
    }

    println!("{}", String::from("2017 AOC #4"));
    graph.find_root_node();
    println!("Part One: {:?}", graph.root_node());
    println!("Part Two: {:?}", graph.find_error());
    // expecting 596
}
