mod input;
static PATH: &'static str = "input/08.txt";

use std::collections::HashMap;
use std::cmp;

fn solve() -> (i32, i32) {
    let input = input::get(PATH);
    let mut register: HashMap<&str, i32> = HashMap::new();
    let mut global_max: i32 = 0;
    for line in input.lines() {
        let mut iter: std::str::SplitWhitespace = line.split_whitespace();
        let name: &str = iter.next().unwrap();
        let verb: &str = iter.next().unwrap();
        let change: i32 = iter.next().unwrap().parse::<i32>().unwrap();
        iter.next(); //if

        let cond_left: i32 = *register.entry(iter.next().unwrap()).or_insert(0);

        let compare: fn(i32, i32) -> bool = match iter.next().unwrap() {
            "==" => |i: i32, j: i32| i == j,
            ">" => |i: i32, j: i32| i > j,
            "<" => |i: i32, j: i32| i < j,
            ">=" => |i: i32, j: i32| i >= j,
            "<=" => |i: i32, j: i32| i <= j,
            "!=" => |i: i32, j: i32| i != j,
            _ => |_, _| true,
        };

        let cond_right: i32 = iter.next().unwrap().parse::<i32>().unwrap();

        if compare(cond_left, cond_right) {
            let ptr: &mut i32 = register.entry(name).or_insert(0);
            match verb {
                "inc" => *ptr += change,
                "dec" => *ptr -= change,
                _ => continue,
            }
        };
        let local_max: i32 = *register.values().max().unwrap();
        global_max = cmp::max(global_max, local_max);
    }
    return (*register.values().max().unwrap(), global_max);
}

fn main() {
    println!("{}", String::from("2017 AOC #8"));
    println!("Part One: {:?}", solve().0);
    println!("Part Two: {:?}", solve().1);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        assert_eq!(solve().0, 4877)
    }
    #[test]
    fn part_two() {
        assert_eq!(solve().1, 5471)
    }
}
