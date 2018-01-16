mod input;
static PATH: &'static str = "input/01.txt";

fn zip_with_next(input_string: String) -> u32 {
    let ring: Vec<u32> = input_string
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let val1 = ring.iter()
        .cycle()
        .skip(1)
        .zip(ring.iter())
        .fold(0, |acc, (i, j)| if i == j { acc + i } else { acc });
    return val1;
}

fn zip_with_halfway(input_string: String) -> u32 {
    let ring: Vec<u32> = input_string
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let val2 = ring.iter()
        .cycle()
        .skip(ring.len() / 2)
        .zip(ring.iter())
        .fold(0, |acc, (i, j)| if i == j { acc + i } else { acc });
    return val2;
}

fn main() {
    println!("2017 AOC #1");
    println!("Part One: {}", zip_with_next(input::get(PATH)));
    println!("Part Two: {}", zip_with_halfway(input::get(PATH)));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        assert_eq!(zip_with_next(input::get(PATH)), 1216);
    }
    #[test]
    fn part_two() {
        assert_eq!(zip_with_halfway(input::get(PATH)), 1072);
    }
}
