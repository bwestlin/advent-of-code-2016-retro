extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<Room>;

struct Room {
    name: String,
    sector_id: u32,
    checksum: String
}

impl Room {
    fn valid_checksum(&self) -> bool {
        let expected_checksum = self.name.chars()
            // Store character count for every character {a -> 2, b-> 2, c -> 1, ...}
            .fold(HashMap::new(), |mut memo: HashMap<char, usize>, c| {
                if c != '-' {
                    *memo.entry(c).or_insert(0) += 1;
                }
                memo
            })
            .iter()
            // Store characters for every character count {2 -> [a, b], 1 -> [c], ...}
            .fold(BTreeMap::new(), |mut memo: BTreeMap<usize, BTreeSet<char>>, (&c, &cnt)| {
                (*memo.entry(cnt).or_insert(BTreeSet::new())).insert(c);
                memo
            })
            .iter().rev()
            // In the order of hightest character count append charcters in alphabethical order
            .fold(String::new(), |mut s, (_, chars)| {
                for &c in chars {
                    if s.len() < 5 {
                        s.push(c);
                    }
                }
                s
            });

        self.checksum.eq(&expected_checksum)
    }

    fn decrypt_name(&self) -> String {
        let mut chars = self.name.chars().collect::<Vec<_>>();

        for i in 0..chars.len() {
            for _ in 0..self.sector_id {
                chars[i] = match chars[i] {
                    '-' | ' ' => ' ',
                    'z'  => 'a',
                    'Z'  => 'A',
                    c  => ((c as u8) + 1) as char,
                }
            }
        }

        chars.iter().collect::<String>()
    }
}

fn part1(input: &Input) -> u32 {
    input.iter()
        .filter(|r| r.valid_checksum())
        .map(|r| r.sector_id)
        .sum()
}

fn part2(input: &Input) -> u32 {
    input.iter()
        .find(|&room| room.decrypt_name().eq("northpole object storage"))
        .map_or(0, |room| room.sector_id)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

impl FromStr for Room {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.*)-(\d+)\[(.*)\]$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let get_s = |idx| caps.get(idx).unwrap().as_str().to_string();
        let get_i = |idx| caps.get(idx).unwrap().as_str().parse::<u32>();
        Ok(Room { name: get_s(1), sector_id: get_i(2)?, checksum: get_s(3) })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap().parse::<Room>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "aaaaa-bbb-z-y-x-123[abxyz]
        a-b-c-d-e-f-g-h-987[abcde]
        not-a-real-room-404[oarel]
        totally-real-room-200[decoy]";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 1514);
    }
}
