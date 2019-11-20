extern crate utils;
extern crate md5;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = String;

fn solve(input: &Input) -> (String, String) {
    const PREFIX: [u8; 3] = [0xFF, 0xFF, 0xF0];
    const HEX: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

    let door_id = input.as_str();
    let mut password1 = String::new();
    let mut password2: Vec<_> = (0..8).map(|_| '_').collect();

    'outer: for index in 0..std::u64::MAX {
        let idx_s = format!("{}", index);
        let digest = md5::compute(format!("{}{}", door_id, idx_s));

        for i in 0..PREFIX.len() {
            if digest[i] & PREFIX[i] != 0 {
                continue 'outer;
            }
        }

        let nibble6 = (digest[2] & 0x0F) as usize;

        if password1.len() < 8 {
            password1.push(HEX[nibble6]);
        }

        if nibble6 < 8 && password2[nibble6] == '_' {
            password2[nibble6] = HEX[((digest[3] >> 4) & 0x0F) as usize];
            if !password2.contains(&'_') {
                break;
            }
        }
    }

    (password1, password2.iter().collect())
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
    Ok(reader.lines().next().unwrap().unwrap())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "abc";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, "18f47a30".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT)).1, "05ace8e3".to_string());
    }
}
