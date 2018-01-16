# aoc-2017-rust
## Advent of Code 2017 Solutions in Rust

I'm doing the [Advent of Code 2017](adventofcode.com) puzzles as a way to learn
more about [Rust](https://www.rust-lang.org/en-US/), so there's no guarantee
that:
  - a) the solutions are right, or
  - b) the Rust is.

That aside, you should be able to pull down this repo and get the solutions up
and running with `rustc`:

```
$ rustc 01.rs
$ ./01
2017 AOC #1
Part One: 1216
Part Two: 1072
```
Each solution also comes with a set of tests:
```
$ rustc --test 01.rs
$ ./01

running 2 tests
test test::part_one ... ok
test test::part_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
