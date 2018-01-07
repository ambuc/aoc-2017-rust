use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn num_steps(mut vec: Vec<i32>, is_neg: bool) -> i32 {
    let mut idx: i32 = 0;
    let mut num_steps: i32 = 0;

    let vec_size: i32 = vec.len() as i32;
    while idx >= (0 as i32) && idx < vec_size {
        let tmp = vec[idx as usize];
        if is_neg && tmp >= 3 {
            vec[idx as usize] -= 1;
        } else {
            vec[idx as usize] += 1;
        }
        idx += tmp;
        num_steps += 1;
    }
    return num_steps;
}

fn main() {
    let path = Path::new("input/05.txt");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    let mut vec: Vec<i32> = Vec::new();
    file.read_to_string(&mut s).unwrap();

    for line in s.lines() {
        vec.push(line.parse::<i32>().unwrap());
    }

    println!("{}", String::from("2017 AOC #5"));
    println!("Part One: {}", num_steps(vec.clone(), false));
    println!("Part Two: {}", num_steps(vec, true));
}
