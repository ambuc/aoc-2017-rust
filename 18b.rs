use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
// use std::time::Duration;

fn to_instructions(file_str: &String) -> Vec<&str> {
    file_str.lines().collect()
}

fn process(instructions: &Vec<&str>) -> i64 {
    let mut register: HashMap<&str, i64> = HashMap::new();
    let mut last_played: i64 = 0;
    let mut idx: i64 = 0;

    while idx >= 0 && idx < (instructions.len() as i64) {
        idx += 1;

        let curr_instr = instructions[idx as usize];
        let mut it = curr_instr.split_whitespace();
        let verb = it.next().unwrap();

        let fst_val_ptr: &str = it.next().unwrap();
        register.entry(fst_val_ptr).or_insert(0);

        if verb == "rcv" {
            if *register.get(fst_val_ptr).unwrap() != 0 {
                return last_played;
            }
        } else if verb == "snd" {
            last_played = *register.get(fst_val_ptr).unwrap();
            continue;
        } else {
            let snd_val: &str = it.next().unwrap();
            let snd: i64 = match snd_val.parse::<i64>() {
                Ok(v) => v,
                Err(_) => *register.entry(snd_val).or_insert(0),
            };

            match verb {
                "set" => {
                    register.insert(fst_val_ptr, snd);
                }
                "add" => {
                    if let Some(fst_val) = register.get_mut(&fst_val_ptr) {
                        *fst_val += snd;
                    }
                }
                "mul" => {
                    if let Some(fst_val) = register.get_mut(&fst_val_ptr) {
                        *fst_val *= snd;
                    }
                }
                "mod" => {
                    if let Some(fst_val) = register.get_mut(&fst_val_ptr) {
                        *fst_val %= snd;
                    }
                }
                "jgz" => {
                    if *register.get(fst_val_ptr).unwrap() > 0 {
                        idx += snd - 1; // to compensate for idx += 1
                    }
                }
                ____ => {
                    panic!("we shouldn't get a verb that doesn't match one of these");
                }
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

    let (_tx_0, _rx_0): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (_tx_1, _rx_1): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    println!("Part One: {:?}", process(&instructions));
}
