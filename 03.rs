use std::cmp;
static INPUT: i32 = 361527;

// accepts a ring number and returns the wing value on that ring
fn east_wing(n: i32) -> i32 {
    return 4 * n * n - 3 * n + 1;
}
fn north_wing(n: i32) -> i32 {
    return 4 * (n + 1) * (n + 1) - 9 * (n + 1) + 6;
}
fn west_wing(n: i32) -> i32 {
    return 4 * (n + 1) * (n + 1) - 7 * (n + 1) + 4;
}
fn south_wing(n: i32) -> i32 {
    return 4 * n * n + 3 * n + 1;
}

// 0 is the center ring, outwards from there
fn ring_number(n: i32) -> i32 {
    return ((n as f32).sqrt().ceil() as i32) / 2;
}

fn distance_to_origin(n: i32) -> i32 {
    let r = ring_number(n);
    let mut dist = 10000000;
    dist = cmp::min(dist, (n - east_wing(r)).abs());
    dist = cmp::min(dist, (n - west_wing(r)).abs());
    dist = cmp::min(dist, (n - north_wing(r)).abs());
    dist = cmp::min(dist, (n - south_wing(r)).abs());
    return r + dist;
}

fn main() {
    println!("{}", String::from("2017 AOC #3"));
    println!("Part One: {}", distance_to_origin(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        assert_eq!(0, distance_to_origin(1));
        assert_eq!(3, distance_to_origin(12));
        assert_eq!(2, distance_to_origin(23));
        assert_eq!(31, distance_to_origin(1024));
        assert_eq!(326, distance_to_origin(INPUT));
    }
}
