use std::collections::HashSet;
use std::collections::HashMap;

fn redistribute(bank: &mut Vec<i32>) {
    let len: usize = bank.len();
    let mut max_idx: usize = 0;
    let mut max_val: i32 = 0;
    for idx in 0..len {
        if bank[idx] > max_val {
            max_idx = idx;
            max_val = bank[idx];
        }
    }
    let cur_idx = max_idx + 1; // get starting index
    let tmp = bank[max_idx] as usize; // save its value
    bank[max_idx] = 0; // clear that register
    for idx in cur_idx..(cur_idx + tmp) {
        //and redistribute its contents
        bank[idx % len] += 1;
    }
}

fn solve() -> (i32, i32) {
    let mut bank: Vec<i32> = vec![10, 3, 15, 10, 5, 15, 5, 15, 9, 2, 5, 8, 5, 2, 3, 6];
    let mut history: HashSet<Vec<i32>> = HashSet::new();
    let mut sequence: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut num_steps: i32 = 0;

    loop {
        if history.contains(&bank) {
            break;
        }
        history.insert(bank.clone());
        sequence.insert(bank.clone(), num_steps);
        num_steps += 1;
        redistribute(&mut bank);
    }

    let cycle_length = num_steps - sequence.get(&bank).unwrap();
    return (num_steps, cycle_length);
}
fn main() {
    println!("{}", String::from("2017 AOC #6"));
    println!("Part One: {}", solve().0);
    println!("Part Two: {}", solve().1);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        assert_eq!(solve().0, 14029);
    }
    #[test]
    fn part_two() {
        assert_eq!(solve().1, 2765);
    }
}
