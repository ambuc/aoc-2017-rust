// skip_length  the number of paces to advance each turn
// capacity     the number of nodes to insert
// value_after  the number which appears before the value we seek to return
fn spinlock(skip_length: usize, capacity: u32, value_after: u32) -> u32 {
    let mut ring: Vec<u32> = Vec::with_capacity(capacity as usize);
    ring.push(0);
    let mut curr_idx: usize = 0;
    for num in 1..(capacity + 1) {
        curr_idx = (curr_idx + skip_length + 1) % ring.len();
        ring.insert(curr_idx, num);
    }
    let mut it = ring.iter();
    let pos = it.position(|&x| x == value_after).unwrap_or(0);
    return ring[pos + 1];
}

// skip_length  the number of paces to advance each turn
// capacity     the number of nodes to insert
//
// default returns the value after (0).
// Since zero is always the first in the list, this really just returns ring[1].
fn spinlock_lite(skip_length: usize, capacity: u32) -> u32 {
    let mut cursor = 0;
    let mut at_one = 0;
    for num in 1..(capacity + 1) {
        cursor = (cursor + skip_length) % (num as usize) + 1;
        if cursor == 1 {
            at_one = num;
        }
    }
    return at_one;
}

fn main() {
    println!("{}", String::from("2017 AOC #16"));
    println!("Part One: {:?}", spinlock(382, 2017, 2017));
    println!("Part Two: {:?}", spinlock_lite(382, 50000000));
}
