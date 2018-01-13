use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;

//    a    b    c    d    e    f    g    h
//    1    2    3    4    5    6    7    8
// 0000 0001 0010 0011 0100 0101 0110 0111
//    i    j    k    l    m    n    o    p
//    9   10   11   12   13   14   15   16
// 1000 1001 1010 1011 1100 1101 1110 1111

#[derive(Clone)]
struct St {
    stage: Vec<char>,
}

impl St {
    fn new() -> St {
        St {
            stage: String::from("abcdefghijklmnop").chars().collect(),
        }
    }

    fn print(self) -> String {
        self.stage.into_iter().collect()
    }

    fn dance(&mut self, steps: &Vec<&str>) {
        for &step in steps {
            &self.step(&step.to_string());
        }
    }

    fn step(&mut self, step: &String) {
        let verb = step.chars().next().unwrap();
        let (_, val) = step.split_at(1);
        match verb {
            's' => {
                let shift: i32 = val.parse::<i32>().unwrap();
                for _ in 0..shift {
                    let tmp = self.stage.pop().unwrap();
                    self.stage.insert(0, tmp);
                }
            }
            'x' => {
                let mut it = val.split('/');
                let a = it.next().unwrap().parse::<i32>().unwrap();
                let b = it.next().unwrap().parse::<i32>().unwrap();
                self.stage.swap(a as usize, b as usize);
            }
            'p' => {
                let mut it = val.split('/');
                let a = it.next().unwrap().chars().next().unwrap();
                let b = it.next().unwrap().chars().next().unwrap();
                let a_ix = self.stage.iter().position(|&x| x == a).unwrap();
                let b_ix = self.stage.iter().position(|&x| x == b).unwrap();
                self.stage.swap(a_ix, b_ix);
            }
            ___ => (),
        }
    }
}

fn cycle_length(steps: &Vec<&str>) -> u32 {
    let mut local_state: St = St::new();
    for i in 1.. {
        local_state.dance(&steps);
        if local_state.stage == St::new().stage {
            return i;
        }
    }
    return 0;
}

fn main() {
    let path = Path::new("input/16.txt");
    let mut file = File::open(&path).unwrap();
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).unwrap();
    let steps: Vec<&str> = file_str.lines().next().unwrap().split(',').collect();

    println!("{}", String::from("2017 AOC #16"));

    let mut state_one: St = St::new();
    state_one.dance(&steps);
    println!("Part One: {:?}", state_one.print());

    let mut state_two: St = St::new();
    for _ in 0..(1000000000 % cycle_length(&steps)) {
        state_two.dance(&steps);
    }
    println!("Part Two: {:?}", state_two.print());
}
