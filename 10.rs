mod knot_hash;
mod input;
static PATH: &'static str = "input/10.txt";

fn solve() -> u32 {
    let windows: Vec<u8> = vec![
        206, 63, 255, 131, 65, 80, 238, 157, 254, 24, 133, 2, 16, 0, 1, 3
    ]; // window sizes
    let mut list: Vec<u32> = (0u32..256).collect(); //vec![0, 1, 2, 3, 4];
    let mut skipsize: usize = 0;
    let mut cursor: usize = 0;
    return knot_hash::run_round(&windows, &mut list, &mut cursor, &mut skipsize);
}

fn main() {
    println!("{}", String::from("2017 AOC #10"));
    println!("Part One: {:?}", solve());
    println!(
        "Part Two: {}",
        knot_hash::knot_hash(input::get(PATH).lines().next().unwrap().to_string())
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve(), 9656);
    }
    #[test]
    fn test_two() {
        assert_eq!(
            knot_hash::knot_hash(input::get(PATH).lines().next().unwrap().to_string()),
            "20b7b54c92bf73cf3e5631458a715149"
        );
    }
}
