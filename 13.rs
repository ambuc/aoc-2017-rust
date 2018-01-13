use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn clean_and_parse_i32(inp: &str) -> i32 {
    inp.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

struct Sweep {
    //takes a time and returns a bool, whether or not we are blocked
    is_caught: Box<Fn(i32) -> bool>,
    depth: i32,
}
impl Sweep {
    fn new(depth: i32) -> Sweep {
        Sweep {
            is_caught: Box::new(move |time| (time) % (2 * (depth - 1)) == 0),
            depth: depth,
        }
    }
}

fn get_severity(firewall: &Vec<(i32, Sweep)>, delay: i32) -> i32 {
    let mut severity = 0;
    for &(idx, ref sweep) in firewall {
        if (sweep.is_caught)(idx + delay) {
            severity += idx * sweep.depth
        }
    }
    severity
}
fn was_caught(firewall: &Vec<(i32, Sweep)>, delay: i32) -> bool {
    for &(idx, ref sweep) in firewall {
        if (sweep.is_caught)(idx + delay) {
            return true;
        }
    }
    false
}

fn perfect_delay(firewall: &Vec<(i32, Sweep)>) -> i32 {
    for i in 0.. {
        if !was_caught(&firewall, i) {
            return i;
        }
    }
    0
}

fn main() {
    let path = Path::new("input/13.txt");
    //let path = Path::new("input/13-test.txt");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    let mut firewall: Vec<(i32, Sweep)> = Vec::new();
    file.read_to_string(&mut s).unwrap();

    for line in s.lines() {
        let mut it = line.split_whitespace();
        let idx = clean_and_parse_i32(it.next().unwrap());
        let depth = clean_and_parse_i32(it.next().unwrap());
        firewall.push((idx, Sweep::new(depth)));
    }

    println!("{}", String::from("2017 AOC #13"));
    println!("Part One: {:?}", get_severity(&firewall, 0));
    println!("Part Two: {:?}", perfect_delay(&firewall));
}
