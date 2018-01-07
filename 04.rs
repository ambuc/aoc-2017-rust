use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let path = Path::new("input/04.txt");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    let mut num_valid_a: i32 = 0;
    file.read_to_string(&mut s).unwrap();
    'lineloop_a: for line in s.lines() {
        let mut set_a = HashSet::new();
        for word in line.split_whitespace() {
            if set_a.contains(word) {
                continue 'lineloop_a;
            } else {
                set_a.insert(word);
            }
        }
        num_valid_a += 1;
    }

    println!("{}", String::from("2017 AOC #4"));
    println!("Part One: {}", num_valid_a);

    let mut num_valid_b: i32 = 0;
    'lineloop_b: for line in s.lines() {
        let mut set_b = HashSet::new();
        for word in line.split_whitespace() {
            let mut tmp: Vec<char> = word.chars().collect();
            tmp.sort();
            if set_b.contains(&tmp) {
                continue 'lineloop_b;
            } else {
                set_b.insert(tmp);
            }
        }
        num_valid_b += 1;
    }

    println!("Part Two: {}", num_valid_b);
}
