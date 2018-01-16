mod input;
static PATH: &'static str = "input/05.txt";

fn num_steps(input: String, is_neg: bool) -> i32 {
    let mut nums = Vec::new();
    for line in input.lines() {
        nums.push(line.parse::<i32>().unwrap());
    }
    let mut idx: i32 = 0;
    let mut num_steps: i32 = 0;
    let size: i32 = nums.len() as i32;
    while idx >= (0 as i32) && idx < size {
        let tmp = nums[idx as usize];
        if is_neg && tmp >= 3 {
            nums[idx as usize] -= 1;
        } else {
            nums[idx as usize] += 1;
        }
        idx += tmp;
        num_steps += 1;
    }
    return num_steps;
}

fn main() {
    println!("{}", String::from("2017 AOC #5"));
    println!("Part One: {}", num_steps(input::get(PATH), false));
    println!("Part Two: {}", num_steps(input::get(PATH), true));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        assert_eq!(num_steps(input::get(PATH), false), 375042);
    }
    #[test]
    fn part_two() {
        assert_eq!(num_steps(input::get(PATH), true), 28707598);
    }
}
