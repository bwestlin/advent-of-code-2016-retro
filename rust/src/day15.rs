extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<Disc>;

#[derive(Clone, Debug)]
struct Disc {
    positions: usize,
    initial: usize
}

impl Disc {
    fn parse(s: &str) -> Disc {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^.*has (\d+).*position (\d+)\.$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Disc { positions: caps[1].parse::<usize>().unwrap(), initial: caps[2].parse::<usize>().unwrap() }
    }

    fn pos(&self, time: usize) -> usize {
        (self.initial + time) % self.positions
    }
}

fn solve(discs: &Input) -> (usize, usize) {
    let start_t = (0..discs[0].positions).find(|t| discs[0].pos(t + 1) == 0).unwrap();
    let step_t = discs[0].positions;
    let extra_disc = Disc { positions: 11, initial: 0 };

    let mut p1 = None;

    for t in (start_t..).step_by(step_t) {

        let all_open = discs.iter().enumerate().all(|(i, d)| d.pos(t + i + 1) == 0);

        if all_open && p1.is_none() {
            p1 = Some(t);
        }

        if all_open && extra_disc.pos(t + discs.len() + 1) == 0 {
            return (p1.unwrap(), t);
        }
    }
    (0, 0)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| Disc::parse(&l.unwrap())).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "Disc #1 has 5 positions; at time=0, it is at position 4.
        Disc #2 has 2 positions; at time=0, it is at position 1.";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, 5);
    }
}
