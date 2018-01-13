mod knot_hash;

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
        knot_hash::run_round(&windows, &mut list, &mut cursor, &mut skipsize)
    );

    let input_str: String = String::from("206,63,255,131,65,80,238,157,254,24,133,2,16,0,1,3");
    println!("Part Two: {}", knot_hash::knot_hash(input_str));
}
