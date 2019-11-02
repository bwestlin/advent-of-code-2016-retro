extern crate utils;

use std::env;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<TSides>;
type TSides = [u32; 3];

fn valid_triangle(s1: u32, s2: u32, s3: u32) -> bool {
    (s1 + s2 > s3) && (s1 + s3 > s2) && (s2 + s3 > s1)
}

fn part1(input: &Input) -> usize {
    input.iter()
        .filter(|t| valid_triangle(t[0], t[1], t[2]))
        .count()
}

fn part2(input: &Input) -> usize {
    input.chunks_exact(3)
        .map(|ts| {
            (0..3).filter(|&i| valid_triangle(ts[0][i], ts[1][i], ts[2][i])).count()
        })
        .sum()
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    fn parse_sides(s: String) -> Result<TSides, ParseIntError> {
        let sides: Vec<_> = s.split(' ')
            .map(|s| s.trim())
            .filter(|&s| s.len() > 0)
            .collect();
        let parse = |s: &str| s.parse::<u32>();
        Ok([parse(sides[0])?, parse(sides[1])?, parse(sides[2])?])
    }
    Ok(reader.lines().flatten().map(parse_sides).flatten().collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "5  10  25
        6  8   10";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 1);
    }
}
