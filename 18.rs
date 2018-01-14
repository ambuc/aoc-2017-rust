use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn process(file_str: String) -> i64 {
    let instructions: Vec<&str> = file_str.lines().collect();
    let mut register: HashMap<&str, i64> = HashMap::new();
    let mut last_played: i64 = 0;
    let mut idx: i64 = 0;

    while idx >= 0 && idx < (instructions.len() as i64) {
        let mut instr = instructions[idx as usize].split_whitespace();
        let verb = instr.next().unwrap();
        let ex_ptr: &str = instr.next().unwrap();
        idx += 1;

        if !register.contains_key(ex_ptr) {
            register.insert(ex_ptr, 0);
        }

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
                Err(_) => {
                    if !register.contains_key(why_val) {
                        register.insert(why_val, 0);
                    }
                    *register.get(why_val).unwrap()
                }
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
    let path = Path::new("input/18.txt");
    //let path = Path::new("input/18-test.txt");
    let mut file_str: String = String::new();
    File::open(&path)
        .unwrap()
        .read_to_string(&mut file_str)
        .unwrap();

    println!("{}", String::from("2017 AOC #16"));
    println!("Part One: {:?}", process(file_str));
}
