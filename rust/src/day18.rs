extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = String;

const TRAP: char = '^';
const SAFE: char = '.';

fn next_row(row: &Vec<char>) -> Vec<char> {
    let len = row.len();
    let mut next = Vec::with_capacity(len);

    for i in 0..len {
        let left   = if i == 0 { '.' } else { row[i - 1] };
        let center = row[i];
        let right  = if i == len - 1 { '.' } else { row[i + 1] };

        let tile = match [left, center, right] {
            [TRAP, TRAP, SAFE] => TRAP,
            [SAFE, TRAP, TRAP] => TRAP,
            [TRAP, SAFE, SAFE] => TRAP,
            [SAFE, SAFE, TRAP] => TRAP,
            _ => SAFE
        };

        next.push(tile);
    }

    next
}

fn count_safe_tiles_row(row: &Vec<char>) -> usize {
    row.iter().filter(|&c| *c == SAFE).count()
}

fn count_safe_tiles(input: &Input, rows: usize) -> usize {
    let mut n_safe = 0;
    let mut latest_row = input.chars().collect::<Vec<_>>();

    n_safe += count_safe_tiles_row(&latest_row);

    for _ in 0..(rows - 1) {
        latest_row = next_row(&latest_row);
        n_safe += count_safe_tiles_row(&latest_row);
    }

    n_safe
}

fn part1(input: &Input) -> usize {
    count_safe_tiles(input, 40)
}

fn part2(input: &Input) -> usize {
    count_safe_tiles(input, 400000)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_count_safe_tiles() {
        assert_eq!(count_safe_tiles(&as_input("..^^."), 3), 6);
        assert_eq!(count_safe_tiles(&as_input(".^^.^.^^^^"), 10), 38);
    }
}
