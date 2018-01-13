struct Gen {
    value: u64,
    factor: u64,
    filter: Option<u64>,
}

impl Gen {
    fn new(init: u64, factor: u64) -> Gen {
        Gen {
            value: init,
            factor: factor,
            filter: None,
        }
    }
    fn new_filtered(init: u64, factor: u64, filter: u64) -> Gen {
        Gen {
            value: init,
            factor: factor,
            filter: Some(filter),
        }
    }
    fn next(&mut self) -> u64 {
        let tmp = self.value * self.factor;
        self.value = tmp;
        // discarding the higher part of the number
        // we'll never return anyway
        self.value %= 2147483647;
        let cand = tmp % 2147483647;
        if cand % self.filter.unwrap_or(1) == 0 {
            return cand;
        } else {
            return self.next();
        }
    }
}

fn to_16(input: u64) -> String {
    let mut s = format!("{:b}", input);
    while s.len() > 16 {
        s.remove(0);
    }
    while s.len() < 16 {
        s.insert(0, '0');
    }
    return s;
}

fn tally_after(gen_a: &mut Gen, gen_b: &mut Gen, limit: u64) -> u64 {
    let mut tally: u64 = 0;
    for _ in 1..limit {
        if to_16(gen_a.next()) == to_16(gen_b.next()) {
            tally += 1;
        }
    }
    return tally;
}

fn main() {
    println!("{}", String::from("2017 AOC #15"));

    //test values
    //let mut gen_a: Gen = Gen::new(65, 16807);
    //let mut gen_b: Gen = Gen::new(8921, 48271);

    println!(
        "Part One: {}",
        tally_after(
            &mut Gen::new(703, 16807),
            &mut Gen::new(516, 48271),
            40000000
        )
    );

    println!(
        "Part Two: {}",
        tally_after(
            &mut Gen::new_filtered(703, 16807, 4),
            &mut Gen::new_filtered(516, 48271, 8),
            5000000
        )
    );
}
