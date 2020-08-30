extern crate utils;

use std::env;
use std::cmp;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<IpBlock>;

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Clone, Copy)]
struct IpBlock {
    from: u32,
    to: u32
}

impl IpBlock {
    fn new(from: u32, to: u32) -> Self {
        IpBlock { from, to }
    }

    fn intersects(&self, other: &IpBlock) -> bool {
        (self.from >= other.from && self.from <= other.to) || (self.to >= other.from && self.to <= other.to) ||
        (other.from >= self.from && other.from <= self.to) || (other.to >= self.from && other.to <= self.to)
    }

    fn adjacent(&self, other: &IpBlock) -> bool {
        self.from == other.to + 1 || other.from == self.to + 1
    }

    fn combine(&self, other: &IpBlock) -> IpBlock {
        IpBlock { from: cmp::min(self.from, other.from), to: cmp::max(self.to, other.to) }
    }

    fn num_blocked(&self) -> u32 {
        self.to - self.from + 1
    }

    fn merge(blocks: &Vec<IpBlock>) -> Vec<IpBlock> {
        let mut blocks = blocks.clone();
        blocks.sort();

        let (mut merged, last) = blocks.iter()
            .fold((vec![], None), |(mut merged, last): (Vec<IpBlock>, Option<IpBlock>), block| {
                if let Some(last) = last {
                    if last.intersects(block) || last.adjacent(block){
                        (merged, Some(last.combine(block)))
                    } else {
                        merged.push(last);
                        (merged, Some(*block))
                    }
                } else {
                    (merged, Some(*block))
                }
            });

        merged.push(last.unwrap());
        merged
    }
}

fn solve(input: &Input) -> (u32, u32) {
    let blocks = IpBlock::merge(input);
    (
        blocks[0].to + 1,
        std::u32::MAX - blocks.iter().map(IpBlock::num_blocked).sum::<u32>() + 1
    )
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

impl FromStr for IpBlock {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        Ok(IpBlock::new(
            split.next().unwrap().parse::<u32>()?,
            split.next().unwrap().parse::<u32>()?
        ))
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap().parse::<IpBlock>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "5-8
        0-2
        4-7";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_ipblock_intersects() {
        assert_eq!(IpBlock::new(5, 6).intersects(&IpBlock::new(4, 9)), true);
        assert_eq!(IpBlock::new(4, 9).intersects(&IpBlock::new(5, 6)), true);
        assert_eq!(IpBlock::new(2, 4).intersects(&IpBlock::new(5, 8)), false);
    }

    #[test]
    fn test_ipblock_adjacent() {
        assert_eq!(IpBlock::new(5, 6).adjacent(&IpBlock::new(7, 9)), true);
        assert_eq!(IpBlock::new(7, 9).adjacent(&IpBlock::new(5, 6)), true);
        assert_eq!(IpBlock::new(2, 4).adjacent(&IpBlock::new(6, 8)), false);
        assert_eq!(IpBlock::new(2, 4).adjacent(&IpBlock::new(3, 5)), false);
    }

    #[test]
    fn test_ipblock_merge() {
        assert_eq!(IpBlock::merge(&as_input(INPUT)),
            as_input(
                "0-2
                 4-8"));
        assert_eq!(IpBlock::merge(
            &as_input(
                "5-7
                 8-9")),
            as_input(
                "5-9"));
        assert_eq!(IpBlock::merge(
            &as_input(
                "5-6
                 7-8
                 4-9
                 9-11
                 15-18
                 18-20")),
            as_input(
                "4-11
                 15-20"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT)).1, std::u32::MAX - 8 + 1);
    }
}
