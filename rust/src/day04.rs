extern crate utils;

use std::env;
use std::cmp;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Room>;

struct Room {
    name: Vec<u8>,
    sector_id: u32,
    checksum: Vec<u8>
}

const LC_A: u8 = 'a' as u8;
const LC_Z: u8 = 'z' as u8;
const ALPHABET_LEN: u32 = ((LC_Z - LC_A) + 1) as u32;

impl Room {
    fn valid_checksum(&self) -> bool {
        let mut name = self.name.clone();
        let mut counts = [0_u8; 128];
        let mut max_dupes = 0;

        name.sort();
        for c in name {
            let c = c as usize;
            counts[c] += 1;
            max_dupes = cmp::max(counts[c], max_dupes);
        }

        let mut cidx = 0;
        'outer: for i in 0..max_dupes {
            let cnts = max_dupes - i;
            for c in LC_A..=LC_Z {
                if counts[c as usize] == cnts {
                    if c != self.checksum[cidx] {
                        return false;
                    }
                    cidx += 1;
                    if cidx >= 5 {
                        break 'outer;
                    }
                }
            }
        }

        true
    }

    fn decrypted_name_is(&self, expected: &[u8]) -> bool {
        let name = &self.name;
        if name.len() != expected.len() {
            return false;
        }

        for i in 0..name.len() {
            let c = name[i];

            let c =
                if c >= LC_A {
                    LC_A + ((self.sector_id + (c - LC_A) as u32) % ALPHABET_LEN) as u8
                } else {
                    ' ' as u8
                };

            if c != expected[i] {
                return false;
            }
        }

        true
    }
}

fn part1(input: &Input) -> u32 {
    input.iter()
        .filter(|r| r.valid_checksum())
        .map(|r| r.sector_id)
        .sum()
}

fn part2(input: &Input) -> u32 {
    let expected_name = "northpole object storage".as_bytes();
    input.iter()
        .find(|&room| room.decrypted_name_is(expected_name))
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
        let sector_id_sidx = s.rfind('-').unwrap();
        let checksum_sidx = s.rfind('[').unwrap();
        Ok(Room {
            name: s[0..sector_id_sidx].as_bytes().into(),
            sector_id: s[(sector_id_sidx + 1)..checksum_sidx].parse::<u32>()?,
            checksum: s[(checksum_sidx + 1)..(checksum_sidx + 6)].as_bytes().into()
        })
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
