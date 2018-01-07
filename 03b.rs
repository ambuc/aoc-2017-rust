use std::collections::HashMap;

fn rotate_vec(v: (i32, i32)) -> (i32, i32) {
    match v {
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        _ => (0, 0),
    }
}

fn add_tuples(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    return (a.0 + b.0, a.1 + b.1);
}

// return an array of size 8! known length
fn get_neighbors(xy: (i32, i32)) -> [(i32, i32); 8] {
    let (x, y) = xy;
    return [
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
    ];
}

fn main() {
    let mut spiral = HashMap::new();
    let input: i32 = 361527;
    let mut cur_pos: (i32, i32) = (0, 0);
    let mut fwd_vec: (i32, i32) = (1, 0);
    let mut left_vec: (i32, i32) = (0, 1);
    let mut last: i32 = 1;

    spiral.insert((0, 0), 1);

    while last < input {
        let to_my_left = add_tuples(cur_pos, left_vec);
        if !spiral.contains_key(&to_my_left) {
            fwd_vec = rotate_vec(fwd_vec);
            left_vec = rotate_vec(left_vec);
        }
        cur_pos = add_tuples(cur_pos, fwd_vec);
        let mut tmp_sum = 0;
        for coord in &get_neighbors(cur_pos) {
            match spiral.get(&coord) {
                Some(val) => tmp_sum += val,
                _ => continue,
            }
        }
        spiral.insert(cur_pos, tmp_sum);
        last = tmp_sum;
    }

    println!("{}", String::from("2017 AOC #3b"));
    println!("Part Two: {}", last);
}
