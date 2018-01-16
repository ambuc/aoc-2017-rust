mod input;
static PATH: &'static str = "input/02.txt";

fn checksum(input_string: String) -> i32 {
    let mut ans: i32 = 0;
    for row_string in input_string.lines() {
        let mut row_vec: Vec<i32> = Vec::new();
        for num in row_string.split("\t") {
            row_vec.push(num.parse::<i32>().unwrap());
        }
        let maximum = row_vec.iter().max().unwrap();
        let minimum = row_vec.iter().min().unwrap();
        ans += maximum - minimum;
    }
    return ans;
}

fn sum_of_row(input_string: String) -> i32 {
    let mut ans: i32 = 0;
    for row_string in input_string.lines() {
        let v: Vec<i32> = row_string
            .split("\t")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        for j in 0..v.len() {
            for i in 0..j {
                if v[i] % v[j] == 0 {
                    ans += v[i] / v[j];
                }
                if v[j] % v[i] == 0 {
                    ans += v[j] / v[i];
                }
            }
        }
    }
    return ans;
}

fn main() {
    println!("{}", String::from("2017 AOC #2"));
    println!("Part One: {}", checksum(input::get(PATH)));
    println!("Part One: {}", sum_of_row(input::get(PATH)));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        assert_eq!(checksum(input::get(PATH)), 53978);
    }
    #[test]
    fn part_two() {
        assert_eq!(sum_of_row(input::get(PATH)), 314);
    }
}
