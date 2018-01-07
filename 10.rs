fn densen(sparse: &Vec<u32>) -> String {
    let mut result: String = String::new();
    for i in sparse.chunks(16) {
        result += &format!("{:x}", i.iter().fold(0, |acc, i| acc ^ i));
    }
    return result;
}

fn knot_hash(input: &'static str) -> String {
    let mut windows_seq: Vec<u8> = String::from(input).into_bytes();
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

fn run_round(
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

fn main() {
    let windows: Vec<u8> = vec![
        206, 63, 255, 131, 65, 80, 238, 157, 254, 24, 133, 2, 16, 0, 1, 3
    ]; // window sizes
    let mut list: Vec<u32> = (0u32..256).collect(); //vec![0, 1, 2, 3, 4];
    let mut skipsize: usize = 0;
    let mut cursor: usize = 0;

    println!("{}", String::from("2017 AOC #10"));
    println!(
        "Part One: {:?}",
        run_round(&windows, &mut list, &mut cursor, &mut skipsize)
    );

    let input_str: &'static str = "206,63,255,131,65,80,238,157,254,24,133,2,16,0,1,3";
    println!("Part Two: {}", knot_hash(input_str));
}
