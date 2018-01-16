use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

// use std::sync::mpsc::{Sender, Receiver};
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

fn to_instructions(file_str: &String) -> Vec<&str> {
    file_str.lines().collect()
}

fn process(instructions: &Vec<&str>) -> i64 {
    let mut register: HashMap<&str, i64> = HashMap::new();
    let mut last_played: i64 = 0;
    let mut idx: i64 = 0;

    while idx >= 0 && idx < (instructions.len() as i64) {
        let mut instr = instructions[idx as usize].split_whitespace();
        let verb = instr.next().unwrap();
        let ex_ptr: &str = instr.next().unwrap();
        idx += 1;

        register.entry(ex_ptr).or_insert(0);

        if verb == "snd" {
            last_played = *register.get(ex_ptr).unwrap();
        } else if verb == "rcv" {
            if *register.get(ex_ptr).unwrap() != 0 {
                return last_played;
            }
        } else {
            let why_val: &str = instr.next().unwrap();
            let why: i64 = match why_val.parse::<i64>() {
                Ok(v) => v,
                Err(_) => *register.entry(why_val).or_insert(0),
            };
            match verb {
                "set" => {
                    register.insert(ex_ptr, why);
                }
                "add" => {
                    if let Some(ex) = register.get_mut(&ex_ptr) {
                        *ex += why;
                    }
                }
                "mul" => {
                    if let Some(ex) = register.get_mut(&ex_ptr) {
                        *ex *= why;
                    }
                }
                "mod" => {
                    if let Some(ex) = register.get_mut(&ex_ptr) {
                        *ex %= why;
                    }
                }
                "jgz" => {
                    let ex = *register.get(ex_ptr).unwrap();
                    if ex > 0 {
                        idx += why - 1; // to compensate for idx += 1
                    }
                }
                ____ => (),
            }
        }
    }
    return 0;
}

fn main() {
    let mut input_str: String = String::new();
    File::open(&Path::new("input/18.txt"))
        .unwrap()
        .read_to_string(&mut input_str)
        .unwrap();

    println!("{}", String::from("2017 AOC #16"));
    let instructions: Vec<&str> = to_instructions(&input_str);

    // let (_tx_0, _rx_0): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    // let (_tx_1, _rx_1): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    println!("Part One: {:?}", process(&instructions));
}
