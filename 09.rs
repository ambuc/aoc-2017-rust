use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn evaluate(input: &str) -> (u32, u32) {
    let mut total: u32 = 0;
    let mut curr: u32 = 1;
    let mut is_skip: bool = false;
    let mut in_garbage: bool = false;
    let mut garbage_count: u32 = 0;
    for chr in input.chars() {
        if is_skip {
            is_skip = false;
            continue;
        }
        if chr == '!' {
            is_skip = true;
            continue;
        }
        if chr == '<' && !in_garbage {
            in_garbage = true;
            continue;
        }
        if chr == '>' {
            in_garbage = false;
        }
        if !in_garbage {
            if chr == '{' {
                total += curr;
                curr += 1;
            }
            if chr == '}' {
                curr -= 1;
            }
        } else {
            garbage_count += 1;
        }
    }
    return (total, garbage_count);
}

#[cfg(not(test))]
fn main() {
    let path = Path::new("input/09.txt");
    let mut file = File::open(&path).unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string)
        .expect("unable to read the file");
    println!("{}", String::from("2017 AOC #9"));
    let (total, garbage_count) = evaluate(&file_string);
    println!("Part One: {:?}", total);
    println!("Part Two: {:?}", garbage_count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_group() {
        let one_group = String::from("{}");
        assert_eq!(evaluate(&one_group).0, 1);
    }
    #[test]
    fn test_three_groups() {
        let three_groups = String::from("{{{}}}");
        assert_eq!(evaluate(&three_groups).0, 6);
    }
    #[test]
    fn test_also_three_groups() {
        let also_three_groups = String::from("{{},{}}");
        assert_eq!(evaluate(&also_three_groups).0, 5);
    }
    #[test]
    fn test_six_groups() {
        let six_groups = String::from("{{{},{},{{}}}}");
        assert_eq!(evaluate(&six_groups).0, 16);
    }
    #[test]
    fn test_four_a_groups() {
        let four_a_groups = String::from("{<a>,<a>,<a>,<a>}");
        assert_eq!(evaluate(&four_a_groups).0, 1);
    }
    #[test]
    fn test_four_ab_groups() {
        let four_ab_groups = String::from("{{<ab>},{<ab>},{<ab>},{<ab>}}");
        assert_eq!(evaluate(&four_ab_groups).0, 9);
    }
    #[test]
    fn test_four_mark_groups() {
        let four_mark_groups = String::from("{{<!!>},{<!!>},{<!!>},{<!!>}}");
        assert_eq!(evaluate(&four_mark_groups).0, 9);
    }
    #[test]
    fn test_four_amark_groups() {
        let four_amark_groups = String::from("{{<a!>},{<a!>},{<a!>},{<ab>}}");
        assert_eq!(evaluate(&four_amark_groups).0, 3);
    }
    #[test]
    fn test_no_garbage() {
        let no_garbage = String::from("<>");
        assert_eq!(evaluate(&no_garbage).1, 0);
    }
    #[test]
    fn test_seventeen_garbage() {
        let seventeen_garbage = String::from("<random characters>");
        assert_eq!(evaluate(&seventeen_garbage).1, 17);
    }
    #[test]
    fn test_three_garbage() {
        let three_garbage = String::from("<<<<>");
        assert_eq!(evaluate(&three_garbage).1, 3);
    }
    #[test]
    fn test_two_garbage() {
        let two_garbage = String::from("<{!>}>");
        assert_eq!(evaluate(&two_garbage).1, 2);
    }
    #[test]
    fn test_doublemark_garbage() {
        let doublemark_garbage = String::from("<!!>");
        assert_eq!(evaluate(&doublemark_garbage).1, 0);
    }
    #[test]
    fn test_triplemark_garbage() {
        let triplemark_garbage = String::from("<!!!>>");
        assert_eq!(evaluate(&triplemark_garbage).1, 0);
    }
    #[test]
    fn test_really_bad_garbage() {
        let really_bad_garbage = String::from("<{o\"i!a,<{i<a>");
        assert_eq!(evaluate(&really_bad_garbage).1, 10);
    }
}
