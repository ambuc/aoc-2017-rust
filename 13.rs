mod input;
static PATH: &'static str = "input/13.txt";

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

fn make_firewall(input: String) -> Vec<(i32, Sweep)> {
    let mut firewall: Vec<(i32, Sweep)> = Vec::new();

    for line in input.lines() {
        let mut it = line.split_whitespace();
        let idx = clean_and_parse_i32(it.next().unwrap());
        let depth = clean_and_parse_i32(it.next().unwrap());
        firewall.push((idx, Sweep::new(depth)));
    }
    return firewall;
}

fn main() {
    println!("{}", String::from("2017 AOC #13"));
    println!(
        "Part One: {:?}",
        get_severity(&make_firewall(input::get(PATH)), 0)
    );
    println!(
        "Part Two: {:?}",
        perfect_delay(&make_firewall(input::get(PATH)))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(get_severity(&make_firewall(input::get(PATH)), 0), 1632);
    }
    #[test]
    fn test_two() {
        assert_eq!(perfect_delay(&make_firewall(input::get(PATH))), 3834136);
    }
}
