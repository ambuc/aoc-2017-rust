pub fn knot_hash(input: String) -> String {
    let mut windows_seq: Vec<u8> = input.into_bytes();
    let mut seed: Vec<u8> = vec![17, 31, 73, 47, 23];
    windows_seq.append(&mut seed);
    let mut list: Vec<u32> = (0u32..256).collect(); //vec![0, 1, 2, 3, 4];

    let mut skipsize: usize = 0;
    let mut cursor: usize = 0;
    for _ in 0..64 {
        run_round(&windows_seq, &mut list, &mut cursor, &mut skipsize);
        // state preserved between rounds
    }
    return densen(&list);
}

pub fn run_round(
    windows: &Vec<u8>,
    list: &mut Vec<u32>,
    cursor: &mut usize,
    skipsize: &mut usize,
) -> u32 {
    let length: usize = list.len();
    for window in windows {
        let windex = *window as usize; // window is a &u8, needs to be a usize
        for i in 0usize..(windex / 2) {
            list.swap((*cursor + i) % length, (*cursor + windex - i - 1) % length);
        }
        *cursor += windex + *skipsize;
        *cursor %= list.len();
        *skipsize += 1;
    }
    return list[0] * list[1];
}

fn densen(sparse: &Vec<u32>) -> String {
    let mut result: String = String::new();
    for i in sparse.chunks(16) {
        result += &format!("{:02x}", i.iter().fold(0, |acc, i| acc ^ i));
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_empty() {
        assert_eq!(
            knot_hash(String::from("")),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
    }
    #[test]
    fn test_aoc2017() {
        assert_eq!(
            knot_hash(String::from("AoC 2017")),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
    }
    #[test]
    fn test_123() {
        assert_eq!(
            knot_hash(String::from("1,2,3")),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
    }
    #[test]
    fn test_124() {
        assert_eq!(
            knot_hash(String::from("1,2,4")),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
