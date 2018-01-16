mod knot_hash;
use std::collections::HashSet;

#[derive(Debug)]
struct Graph {
    nodes: HashSet<(i32, i32)>,
}
impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashSet::new(),
        }
    }
    fn num_regions(&self) -> i32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut queue: Vec<&(i32, i32)> = self.nodes.iter().collect();
        queue.sort();
        queue.reverse();
        let mut num_regions: i32 = 0;
        while visited.len() < self.nodes.len() {
            let curr = queue.pop().unwrap();
            if visited.contains(&curr) {
                continue;
            } else {
                //println!("{:?}", curr);
                self.visit_neighbors(*curr, &mut visited);
                num_regions += 1;
            }
        }
        num_regions
    }
    fn visit_neighbors(&self, curr: (i32, i32), mut visited: &mut HashSet<(i32, i32)>) {
        visited.insert(curr);
        let (i, j) = curr;
        let north = (i, j + 1);
        let east = (i + 1, j);
        let south = (i, j - 1);
        let west = (i - 1, j);
        for neighbor in vec![north, east, south, west].iter() {
            if !visited.contains(&neighbor) {
                if self.nodes.contains(&neighbor) {
                    self.visit_neighbors(*neighbor, &mut visited);
                }
            }
        }
    }
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn num_zeroes(&self) -> i32 {
        let mut sum: i32 = 0;
        for row in &self.grid {
            for col in row {
                if *col == true {
                    sum += 1;
                }
            }
        }
        sum
    }
    fn to_graph(&self) -> Graph {
        let mut graph: Graph = Graph::new();
        for (ridx, row) in self.grid.iter().enumerate() {
            for (cidx, cell) in row.iter().enumerate() {
                let row_idx = ridx as i32;
                let col_idx = cidx as i32;
                let this = (row_idx, col_idx);
                if *cell == true {
                    graph.nodes.insert(this.clone());
                }
            }
        }
        graph
    }
}

fn hexchar_to_vecbool(ch: char) -> Vec<bool> {
    let num = i32::from_str_radix(&ch.to_string(), 16).unwrap();
    vec![
        num / 8 % 2 != 0,
        num / 4 % 2 != 0,
        num / 2 % 2 != 0,
        num / 1 % 2 != 0,
    ]
}

fn hash_to_vecbool(h: String) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::new();
    for ch in h.chars() {
        vec.append(&mut hexchar_to_vecbool(ch));
    }
    vec
}

fn make_grid(input: &String) -> Grid {
    let mut g: Grid = Grid { grid: Vec::new() };
    for i in 0..128 {
        let rowstring: String = input.clone() + &String::from("-") + &i.to_string();
        g.grid
            .push(hash_to_vecbool(knot_hash::knot_hash(rowstring)));
    }
    return g;
}

fn main() {
    println!("{}", String::from("2017 AOC #14"));
    println!(
        "Part One: {}",
        make_grid(&String::from("stpzcrnm")).num_zeroes()
    );
    println!(
        "Part Two: {}",
        make_grid(&String::from("stpzcrnm"))
            .to_graph()
            .num_regions()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(make_grid(&String::from("stpzcrnm")).num_zeroes(), 8250)
    }
    #[test]
    fn test_two() {
        assert_eq!(
            make_grid(&String::from("stpzcrnm"))
                .to_graph()
                .num_regions(),
            1113
        );
    }
}
