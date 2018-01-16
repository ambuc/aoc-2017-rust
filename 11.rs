mod input;
static PATH: &'static str = "input/11.txt";

use std::cmp;

//   n       -z
// nw ne   +y  +x
// sw se   -x  -y
//   s       +z

struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

fn origin() -> Coord {
    Coord { x: 0, y: 0, z: 0 }
}

fn distance_btw(a: &Coord, b: &Coord) -> i32 {
    let v = vec![(a.x - b.x).abs(), (a.y - b.y).abs(), (a.z - b.z).abs()];
    return *v.iter().max().unwrap() as i32;
}

fn to_coord(steps: &str) -> (Coord, i32) {
    let mut max_distance: i32 = 0;
    let mut c: Coord = Coord { x: 0, y: 0, z: 0 };
    let steps_vec: Vec<&str> = steps.split(',').collect();
    for step in steps_vec {
        match step {
            "n" => {
                c.y += 1;
                c.z -= 1
            }
            "ne" => {
                c.x += 1;
                c.z -= 1
            }
            "se" => {
                c.x += 1;
                c.y -= 1
            }
            "s" => {
                c.z += 1;
                c.y -= 1
            }
            "sw" => {
                c.z += 1;
                c.x -= 1
            }
            "nw" => {
                c.y += 1;
                c.x -= 1
            }
            _ => continue,
        }
        max_distance = cmp::max(max_distance, distance_btw(&origin(), &c));
    }
    return (c, max_distance);
}

fn main() {
    println!("{}", String::from("2017 AOC #11"));
    println!(
        "Part One: {:?}",
        distance_btw(
            &origin(),
            &to_coord(&input::get(PATH).lines().next().unwrap()).0
        )
    );
    println!(
        "Part Two: {:?}",
        &to_coord(&input::get(PATH).lines().next().unwrap()).1
    );
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            distance_btw(
                &origin(),
                &to_coord(&input::get(PATH).lines().next().unwrap()).0
            ),
            764
        )
    }
    #[test]
    fn test_part_two() {
        assert_eq!(to_coord(&input::get(PATH).lines().next().unwrap()).1, 1532)
    }
    #[test]
    fn test_one() {
        let one = String::from("ne,ne,ne");
        assert_eq!(distance_btw(&origin(), &to_coord(&one).0), 3);
    }
    #[test]
    fn test_two() {
        let two = String::from("ne,ne,sw,sw");
        assert_eq!(distance_btw(&origin(), &to_coord(&two).0), 0);
    }
    #[test]
    fn test_three() {
        let three = String::from("ne,ne,s,s");
        assert_eq!(distance_btw(&origin(), &to_coord(&three).0), 2);
    }
    #[test]
    fn test_four() {
        let four = String::from("se,sw,se,sw,sw");
        assert_eq!(distance_btw(&origin(), &to_coord(&four).0), 3);
    }

    #[test]
    fn test_five() {
        let five = String::from("se,se,se,s,s");
        assert_eq!(distance_btw(&origin(), &to_coord(&five).0), 5);
    }

}
