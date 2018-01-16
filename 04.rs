use std::collections::HashSet;

mod input;
static PATH: &'static str = "input/04.txt";

fn valid_a(list: String) -> i32 {
    let mut count = 0;
    'lineloop: for line in list.lines() {
        let mut set_a = HashSet::new();
        for word in line.split_whitespace() {
            if set_a.contains(word) {
                continue 'lineloop;
            } else {
                set_a.insert(word);
            }
        }
        count += 1;
    }
    return count;
}
fn valid_b(list: String) -> i32 {
    let mut count: i32 = 0;
    'lineloop: for line in list.lines() {
        let mut set_b = HashSet::new();
        for word in line.split_whitespace() {
            let mut tmp: Vec<char> = word.chars().collect();
            tmp.sort();
            if set_b.contains(&tmp) {
                continue 'lineloop;
            } else {
                set_b.insert(tmp);
            }
        }
        count += 1;
    }
    return count;
}

fn main() {
    println!("{}", String::from("2017 AOC #4"));
    println!("Part One: {}", valid_a(input::get(PATH)));
    println!("Part Two: {}", valid_b(input::get(PATH)));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        assert_eq!(valid_a(input::get(PATH)), 383);
    }
    #[test]
    fn part_two() {
        assert_eq!(valid_b(input::get(PATH)), 265);
    }
}
